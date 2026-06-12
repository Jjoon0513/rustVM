#![no_std]

const CF: u8 = 1 << 0;
const PF: u8 = 1 << 1;
const AF: u8 = 1 << 2;
const ZF: u8 = 1 << 3;
const SF: u8 = 1 << 4;
const OF: u8 = 1 << 5;

//TODO
/*
R0 ~ R13 (범용)
R14, R15 (DIV/MUL전용)
*/

//  0x0000 ~ 0x00FF       |    256 B     |   Kernel   | 인터럽트/시스콜 벡터 테이블 (MSR 대용)
//  0x0100 ~ 0x9FFF       |   40.7 KB    |    User    | 유저 프로그램 코드 & 데이터 (텍스트, 힙)
//  0xA000 ~ 0xBFFF       |     8 KB     |    User    | 유저 스택 공간 (0xBFFF부터 아래로 감소)
//  0xC000 ~ 0xC0FF       |    256 B     |   Kernel   | MMIO 장치 구역 (하드웨어 I/O 레지스터)
//  0xC100 ~ 0xF000       |   11.7 KB    |   Kernel   | 게스트 커널 소스 코드 및 드라이버
//  0xF001 ~ 0xFFFF       |     4 KB     |   Kernel   | 커널 전용 스택 공간 (0xFFFF부터 아래로 감소)

//  MMIO
//  0xC000 : [WRITE] UART TX
//  0xC001 : [READ]  UART RX
//  0xC002 : [READ]  TIMER
//  0xC003 : [READ]  RANDOM
pub struct Vm {
    // 레지스터
    pub registers: [u16; 16],
    // MSR
    pub lstar: u16,
    pub cpl: u8,
    pub kernel_gs_base: u16,

    pub pc: usize,
    pub memory: [u8; 65536],
    pub flags: u8,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            lstar: 0,
            cpl: 0,
            kernel_gs_base: 0,

            pc: 0,
            memory: [0; 65536],
            flags: 0,
        }
    }

    fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    fn get_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    pub fn get_memory_u8(&self) -> u8 {
        self.memory[self.pc as usize]
    }

    pub fn get_memory_u8_as_u16(&self) -> u16 {
        self.memory[self.pc as usize] as u16
    }

    ///이 함수는 프로그램 카운터를 두칸 앞으로 옮김 `self.pc += 2`
    ///
    /// Advances the program counter by two. `self.pc += 2`
    pub fn add_high_low(&mut self) -> u16 {
        let low = self.get_memory_u8_as_u16();
        self.pc += 1;

        let high = self.get_memory_u8_as_u16();
        self.pc += 1;

        low | (high << 8)
    }

    pub fn step(&mut self) {
        let opcode = self.get_memory_u8();
        self.pc += 1;

        /*
        0x00 ~ 0x07	시스템
        0x08 ~ 0x0F	MOV
        0x10 ~ 0x17	Undecided
        0x18 ~ 0x1F	ADD/SUB
        0x20 ~ 0x27	MUL/DIV
        0x28 ~ 0x2F	AND/OR/XOR
        0x30 ~ 0x37	CMP
        0x38 ~ 0x3F	JMP
        0x40 ~ 0x47	PUSH/POP
        0x48 ~ 0x4F	CALL/RET
        0x50 ~ 0x57	LOAD
        0x58 ~ 0x5F	STORE
        0X60 ~ ...  ETC
                 */

        match opcode {
            0x00 => {} //nop
            0x08 => {
                //movi: movi <Register> <LOWb> <HIGHb> (Lb + Hb -> R)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let value = (&mut *self).add_high_low();

                self.registers[reg as usize] = value;
            }
            0x09 => {
                //movr: movr <Register0> <Register1> (R1 -> R0)
                let reg0 = self.get_memory_u8();
                self.pc += 1;

                let reg1 = self.get_memory_u8();
                self.pc += 1;

                self.registers[reg0 as usize] = self.registers[reg1 as usize];
            }

            0x18 => {
                //addi: addi <Register> <LOWb> <HIGHb> (R += Lb + Hb)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = (&mut *self).add_high_low();

                let rev = self.registers[reg as usize];

                let (rst, carry) = rev.overflowing_add(val);

                self.set_flag(CF, carry);
                self.set_flag(ZF, rst == 0);
                self.set_flag(SF, rst & 0x8000 != 0);
                self.set_flag(OF, ((rev ^ rst) & (val ^ rst) & 0x8000) != 0);

                self.registers[reg as usize] = rst;
            }
            0x19 => {
                //addr: addr <Register0> <Register1> (R0 += R1)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = self.get_memory_u8();
                self.pc += 1;

                let rev = self.registers[reg as usize];

                let vav = self.registers[val as usize];

                let (rst, carry) = rev.overflowing_add(vav);

                self.set_flag(CF, carry);
                self.set_flag(ZF, rst == 0);
                self.set_flag(SF, rst & 0x8000 != 0);
                self.set_flag(OF, ((rev ^ rst) & (vav ^ rst) & 0x8000) != 0);

                self.registers[reg as usize] = rst;
            }

            0x1A => {
                //subi: subi <Register> <LOWb> <HIGHb> (R -= Lb + Hb)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = (&mut *self).add_high_low();

                let rev = self.registers[reg as usize];

                let (rst, borrow) = rev.overflowing_sub(val);

                self.set_flag(CF, borrow);
                self.set_flag(ZF, rst == 0);
                self.set_flag(SF, rst & 0x8000 != 0);
                self.set_flag(OF, ((rev ^ val) & (rev ^ rst) & 0x8000) != 0);

                self.registers[reg as usize] = rst;
            }
            0x1B => {
                //subr: subr <Register0> <Register1> (R0 -= R1)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = self.get_memory_u8();
                self.pc += 1;

                let rev = self.registers[reg as usize];

                let vav = self.registers[val as usize];

                let (rst, borrow) = rev.overflowing_sub(vav);

                self.set_flag(CF, borrow);
                self.set_flag(ZF, rst == 0);
                self.set_flag(SF, rst & 0x8000 != 0);
                self.set_flag(OF, ((rev ^ vav) & (rev ^ rst) & 0x8000) != 0);

                self.registers[reg as usize] = rst;
            }
            0x1C => {
                //cmp: cmp <Register0> <Register1> (R0 - R1)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = self.get_memory_u8();
                self.pc += 1;

                let rev = self.registers[reg as usize];

                let vav = self.registers[val as usize];

                let (rst, borrow) = rev.overflowing_sub(vav);

                self.set_flag(CF, borrow);
                self.set_flag(ZF, rst == 0);
                self.set_flag(SF, rst & 0x8000 != 0);
                self.set_flag(OF, ((rev ^ vav) & (rev ^ rst) & 0x8000) != 0);
            }

            // MULI, MULR, DIVI, DIVR은 특수레지스트리 R15를 주로 연산하고 R14, R15에 결과값을 저장함
            0x20 => {
                // muli <LOWb> + <HIGHb> * <R15> = <R14(LOW)> , <R15(HIGH)>
                let val = (&mut *self).add_high_low();

                let lhs = self.registers[15];
                let full = (lhs as u32) * (val as u32);

                self.registers[14] = (full >> 16) as u16; // H
                self.registers[15] = full as u16; // L

                self.set_flag(CF, full > 0xFFFF);
                self.set_flag(OF, full > 0xFFFF);
                self.set_flag(ZF, full == 0);
                self.set_flag(SF, (self.registers[15] & 0x8000) != 0);
            }

            0x21 => {
                // mulr <Register> * <R15> = <R14(LOW)> , <R15(HIGH)>
                let reg = self.get_memory_u8();
                self.pc += 1;

                let lhs = self.registers[15];
                let rhs = self.registers[reg as usize];

                let full = (lhs as u32) * (rhs as u32);

                self.registers[14] = (full >> 16) as u16; // H
                self.registers[15] = full as u16; // L

                self.set_flag(CF, full > 0xFFFF);
                self.set_flag(OF, full > 0xFFFF);
                self.set_flag(ZF, full == 0);
                self.set_flag(SF, (self.registers[15] & 0x8000) != 0);
            }

            0x22 => {
                // divi <LOWb> + <HIGHb>
                // [R14:R15](32비트) / <즉시값>(16비트) = R15(몫), R14(나머지)
                let val = self.add_high_low();

                // TODO 메모리 점프 (인터럽트)
                if val == 0 {
                    panic!("Divide by Zero Exception!");
                }

                let lhs_high = self.registers[14] as u32;
                let lhs_low = self.registers[15] as u32;
                let full_dividend = (lhs_high << 16) | lhs_low;

                let quotient = full_dividend / (val as u32);
                let remainder = full_dividend % (val as u32);

                // TODO 메모리 점프 (인터럽트)
                if quotient > 0xFFFF {
                    panic!("Divide Overflow Exception!");
                }

                self.registers[15] = quotient as u16;
                self.registers[14] = remainder as u16;

                self.set_flag(ZF, quotient == 0);
                self.set_flag(SF, (self.registers[15] & 0x8000) != 0);
            }

            0x23 => {
                // divr <Register>
                // [R14:R15](32비트) / <Register>(16비트) = R15(몫), R14(나머지)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = self.registers[reg as usize];
                

                // TODO 메모리 점프 (인터럽트)
                if val == 0 {
                    panic!("Divide by Zero Exception!");
                }

                let lhs_high = self.registers[14] as u32;
                let lhs_low = self.registers[15] as u32;
                let full_dividend = (lhs_high << 16) | lhs_low;

                let quotient = full_dividend / (val as u32);
                let remainder = full_dividend % (val as u32);
                

                // TODO 메모리 점프 (인터럽트)
                if quotient > 0xFFFF {
                    panic!("Divide Overflow Exception!");
                }

                self.registers[15] = quotient as u16;
                self.registers[14] = remainder as u16;

                self.set_flag(ZF, quotient == 0);
                self.set_flag(SF, (self.registers[15] & 0x8000) != 0);
            }

            _ => panic!("Unknown opcode"),
        }
    }
}
