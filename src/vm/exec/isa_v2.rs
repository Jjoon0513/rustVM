//! ISA v2 확장 — LLVM/컴파일러 백엔드를 위해 추가된 명령어들 (기존 opcode는 전혀 건드리지 않음).
//! 나도 이제 Rust로 RVA 어셈블리를 뽑을수 있을것인가!?!??
//! 젭라.
//!
//! | opcode | 니모닉  | 피연산자                  | 의미 |
//! |--------|---------|---------------------------|------|
//! | 0x10   | adci    | Reg, Imm16                | R += imm + CF |
//! | 0x11   | adcr    | Reg, Reg                  | R0 += R1 + CF |
//! | 0x12   | sbbi    | Reg, Imm16                | R -= imm + CF |
//! | 0x13   | sbbr    | Reg, Reg                  | R0 -= R1 + CF |
//! | 0x3B   | jmpr    | Reg                       | PC = R |
//! | 0x4A   | callr   | Reg                       | call(R)  (프레임은 `call`과 동일) |
//! | 0x54   | loadb   | Reg dst, Reg addr         | dst = zero_extend(mem8[addr]) |
//! | 0x55   | loado   | Reg dst, Reg base, Imm16  | dst = mem16[base + off] |
//! | 0x56   | loadbo  | Reg dst, Reg base, Imm16  | dst = zero_extend(mem8[base + off]) |
//! | 0x58   | storeb  | Reg addr, Reg src         | mem8[addr] = src & 0xFF |
//! | 0x59   | storeo  | Reg base, Imm16, Reg src  | mem16[base + off] = src |
//! | 0x5A   | storebo | Reg base, Imm16, Reg src  | mem8[base + off] = src & 0xFF |
//! | 0x64   | sari    | Reg, Imm8                 | R = (signed R) >> imm (산술 시프트) |
//! | 0x65   | sarr    | Reg, Reg                  | R0 = (signed R0) >> R1 |
//!
//! 
//!
//! 규칙
//! - `base + off`는 **16비트 래핑** 덧셈이다. off는 부호 있는 16비트로 취급되므로 `-4` 같은 값(0xFFFC)이
//!   `base - 4`로 동작한다. 워드 접근의 둘째 바이트(`ea + 1`)도 래핑되어 호스트 패닉이 나지 않는다.
//! - 주소 0xC000(UART TX)에 대한 store는 기존 `storer/storei`와 같이 커널 전용이며, 유저 모드면 #GP.
//!   워드 store가 0xC000이면 기존과 동일하게 하위 바이트만 UART로 나간다.
//! - adc/sbb의 ZF는 해당 워드의 결과만 본다(x86과 동일). 다중 워드 비교는 소프트웨어에서 처리해야 한다.

use crate::vm::exec::interrupt::Interrupt::GeneralProtection;
use crate::vm::{CF, OF, SF, Vm, ZF};

const UART_TX: u16 = 0xC000;

impl Vm {
    // ───────────────────────── 내부 헬퍼 ─────────────────────────

    /// 레지스터 번호 1바이트를 읽고 PC를 1 증가
    fn fetch_reg(&mut self) -> u8 {
        let r = self.fetch_u8();
        self.pc += 1;
        r
    }

    /// 유효 주소 = base + off (16비트 래핑)
    fn effective_addr(&self, base_reg: u8, off: u16) -> u16 {
        self.registers[base_reg as usize].wrapping_add(off)
    }

    fn read_u8_at(&self, addr: u16) -> u8 {
        self.get_memory(addr as usize)
    }

    fn read_u16_at(&self, addr: u16) -> u16 {
        let low = self.get_memory(addr as usize);
        let high = self.get_memory(addr.wrapping_add(1) as usize);
        self.combine_u8_to_u16_const(low, high)
    }

    fn combine_u8_to_u16_const(&self, low: u8, high: u8) -> u16 {
        low as u16 | ((high as u16) << 8)
    }

    /// 바이트 쓰기. UART 주소면 커널 전용(#GP). 쓰기가 일어났으면 true
    fn write_u8_at(&mut self, addr: u16, value: u8) -> bool {
        if addr == UART_TX {
            if self.cpl != 0 {
                self.interrupt(GeneralProtection as u8);
                return false;
            }
            self.uart_write(value);
            return true;
        }
        self.set_memory(value, addr as usize);
        true
    }

    /// 워드 쓰기 (little endian, 둘째 바이트 주소는 래핑)
    fn write_u16_at(&mut self, addr: u16, value: u16) -> bool {
        if addr == UART_TX {
            // 기존 storer/storei와 동일: 하위 바이트만 UART로
            return self.write_u8_at(addr, value as u8);
        }
        self.set_memory(value as u8, addr as usize);
        self.set_memory((value >> 8) as u8, addr.wrapping_add(1) as usize);
        true
    }

    fn add_with_carry(&mut self, lhs: u16, rhs: u16) -> u16 {
        let carry_in = self.get_flag(CF) as u32;
        let sum = lhs as u32 + rhs as u32 + carry_in;
        let result = sum as u16;
        self.set_flag(CF, sum > 0xFFFF);
        self.set_flag(ZF, result == 0);
        self.set_flag(SF, result & 0x8000 != 0);
        self.set_flag(OF, ((lhs ^ result) & (rhs ^ result) & 0x8000) != 0);
        result
    }

    fn sub_with_borrow(&mut self, lhs: u16, rhs: u16) -> u16 {
        let borrow_in = self.get_flag(CF) as u32;
        let result = lhs.wrapping_sub(rhs).wrapping_sub(borrow_in as u16);
        self.set_flag(CF, (lhs as u32) < rhs as u32 + borrow_in);
        self.set_flag(ZF, result == 0);
        self.set_flag(SF, result & 0x8000 != 0);
        self.set_flag(OF, ((lhs ^ rhs) & (lhs ^ result) & 0x8000) != 0);
        result
    }

    fn arithmetic_shift_right(&mut self, reg: u8, amount: u8) {
        let value = self.registers[reg as usize];
        let shift = amount.min(15) as u32; // 16 이상은 부호 비트로 가득 참
        let result = ((value as i16) >> shift) as u16;

        // 마지막으로 밀려나간 비트 (amount > 16이면 부호 비트)
        let carry = match amount {
            0 => false,
            1..=16 => (value >> (amount - 1)) & 1 != 0,
            _ => value & 0x8000 != 0,
        };

        self.registers[reg as usize] = result;
        self.set_flag(CF, carry);
        self.set_flag(OF, false);
        self.set_flag(ZF, result == 0);
        self.set_flag(SF, result & 0x8000 != 0);
    }

    // ───────────────────────── ADC / SBB ─────────────────────────

    // adci: adci <Register> <LOWb> <HIGHb>
    pub fn adci(&mut self) {
        let reg = self.fetch_reg();
        let imm = self.get_high_low();
        let lhs = self.registers[reg as usize];
        self.registers[reg as usize] = self.add_with_carry(lhs, imm);
    }

    // adcr: adcr <Register0> <Register1>
    pub fn adcr(&mut self) {
        let reg0 = self.fetch_reg();
        let reg1 = self.fetch_reg();
        let lhs = self.registers[reg0 as usize];
        let rhs = self.registers[reg1 as usize];
        self.registers[reg0 as usize] = self.add_with_carry(lhs, rhs);
    }

    // sbbi: sbbi <Register> <LOWb> <HIGHb>
    pub fn sbbi(&mut self) {
        let reg = self.fetch_reg();
        let imm = self.get_high_low();
        let lhs = self.registers[reg as usize];
        self.registers[reg as usize] = self.sub_with_borrow(lhs, imm);
    }

    // sbbr: sbbr <Register0> <Register1>
    pub fn sbbr(&mut self) {
        let reg0 = self.fetch_reg();
        let reg1 = self.fetch_reg();
        let lhs = self.registers[reg0 as usize];
        let rhs = self.registers[reg1 as usize];
        self.registers[reg0 as usize] = self.sub_with_borrow(lhs, rhs);
    }

    // ───────────────────────── SAR ─────────────────────────

    // sari: sari <Register> <Amount>
    pub fn sari(&mut self) {
        let reg = self.fetch_reg();
        let amount = self.fetch_u8();
        self.pc += 1;
        self.arithmetic_shift_right(reg, amount);
    }

    // sarr: sarr <Register0> <Register1>  (amount = R1의 하위 바이트가 아니라 R1 전체, 255로 포화)
    pub fn sarr(&mut self) {
        let reg0 = self.fetch_reg();
        let reg1 = self.fetch_reg();
        let amount = self.registers[reg1 as usize].min(255) as u8;
        self.arithmetic_shift_right(reg0, amount);
    }

    // ───────────────────────── 간접 점프 / 호출 ─────────────────────────

    // jmpr: jmpr <Register>
    pub fn jmpr(&mut self) {
        let reg = self.fetch_reg();
        self.pc = self.registers[reg as usize] as usize;
    }

    // callr: callr <Register>
    pub fn callr(&mut self) {
        let reg = self.fetch_reg();
        let target = self.registers[reg as usize];
        self.call_to(target);
    }

    // ───────────────────────── 바이트 / 오프셋 load·store ─────────────────────────

    // loadb: loadb <dst> <addr_reg>
    pub fn loadb(&mut self) {
        let dst = self.fetch_reg();
        let addr_reg = self.fetch_reg();
        let addr = self.registers[addr_reg as usize];
        self.registers[dst as usize] = self.read_u8_at(addr) as u16;
    }

    // loado: loado <dst> <base_reg> <LOWb> <HIGHb>
    pub fn loado(&mut self) {
        let dst = self.fetch_reg();
        let base = self.fetch_reg();
        let off = self.get_high_low();
        let addr = self.effective_addr(base, off);
        self.registers[dst as usize] = self.read_u16_at(addr);
    }

    // loadbo: loadbo <dst> <base_reg> <LOWb> <HIGHb>
    pub fn loadbo(&mut self) {
        let dst = self.fetch_reg();
        let base = self.fetch_reg();
        let off = self.get_high_low();
        let addr = self.effective_addr(base, off);
        self.registers[dst as usize] = self.read_u8_at(addr) as u16;
    }

    // storeb: storeb <addr_reg> <src>
    pub fn storeb(&mut self) {
        let addr_reg = self.fetch_reg();
        let src = self.fetch_reg();
        let addr = self.registers[addr_reg as usize];
        self.write_u8_at(addr, self.registers[src as usize] as u8);
    }

    // storeo: storeo <base_reg> <LOWb> <HIGHb> <src>
    pub fn storeo(&mut self) {
        let base = self.fetch_reg();
        let off = self.get_high_low();
        let src = self.fetch_reg();
        let addr = self.effective_addr(base, off);
        self.write_u16_at(addr, self.registers[src as usize]);
    }

    // storebo: storebo <base_reg> <LOWb> <HIGHb> <src>
    pub fn storebo(&mut self) {
        let base = self.fetch_reg();
        let off = self.get_high_low();
        let src = self.fetch_reg();
        let addr = self.effective_addr(base, off);
        self.write_u8_at(addr, self.registers[src as usize] as u8);
    }
}


