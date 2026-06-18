use crate::exec::*;
use crate::vm::Vm;

impl Vm {
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
        0x30 ~ 0x37	Undecided
        0x38 ~ 0x3F	JMP
        0x40 ~ 0x47	PUSH/POP
        0x48 ~ 0x4F	CALL/RET
        0x50 ~ 0x57	LOAD
        0x58 ~ 0x5F	STORE
        0X60 ~ ...  ETC
        */

        match opcode {
            0x00 => {} //nop
            0x08 => self.movi(),
            0x09 => self.movr(),

            0x18 => {
                //addi: addi <Register> <LOWb> <HIGHb> (R += Lb + Hb)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = (&mut *self).add_high_low();

                let rev = self.registers[reg as usize];

                let (rst, carry) = rev.overflowing_add(val);

                self.set_flag(crate::vm::CF, carry);
                self.set_flag(crate::vm::ZF, rst == 0);
                self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
                self.set_flag(crate::vm::OF, ((rev ^ rst) & (val ^ rst) & 0x8000) != 0);

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

                self.set_flag(crate::vm::CF, carry);
                self.set_flag(crate::vm::ZF, rst == 0);
                self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
                self.set_flag(crate::vm::OF, ((rev ^ rst) & (vav ^ rst) & 0x8000) != 0);

                self.registers[reg as usize] = rst;
            }

            0x1A => {
                //subi: subi <Register> <LOWb> <HIGHb> (R -= Lb + Hb)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = (&mut *self).add_high_low();

                let rev = self.registers[reg as usize];

                let (rst, borrow) = rev.overflowing_sub(val);

                self.set_flag(crate::vm::CF, borrow);
                self.set_flag(crate::vm::ZF, rst == 0);
                self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
                self.set_flag(crate::vm::OF, ((rev ^ val) & (rev ^ rst) & 0x8000) != 0);

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

                self.set_flag(crate::vm::CF, borrow);
                self.set_flag(crate::vm::ZF, rst == 0);
                self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
                self.set_flag(crate::vm::OF, ((rev ^ vav) & (rev ^ rst) & 0x8000) != 0);

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

                self.set_flag(crate::vm::CF, borrow);
                self.set_flag(crate::vm::ZF, rst == 0);
                self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
                self.set_flag(crate::vm::OF, ((rev ^ vav) & (rev ^ rst) & 0x8000) != 0);
            }

            // MULI, MULR, DIVI, DIVR은 특수레지스트리 R15를 주로 연산하고 R14, R15에 결과값을 저장함
            0x20 => {
                // muli <LOWb> + <HIGHb> * <R15> = <R14(LOW)> , <R15(HIGH)>
                let val = (&mut *self).add_high_low();

                let lhs = self.registers[15];
                let full = (lhs as u32) * (val as u32);

                self.registers[14] = (full >> 16) as u16; // H
                self.registers[15] = full as u16; // L

                self.set_flag(crate::vm::CF, full > 0xFFFF);
                self.set_flag(crate::vm::OF, full > 0xFFFF);
                self.set_flag(crate::vm::ZF, full == 0);
                self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
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

                self.set_flag(crate::vm::CF, full > 0xFFFF);
                self.set_flag(crate::vm::OF, full > 0xFFFF);
                self.set_flag(crate::vm::ZF, full == 0);
                self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
            }

            0x22 => {
                // divi <LOWb> + <HIGHb>
                // [R14:R15](32비트) / <즉시값>(16비트) = R15(몫), R14(나머지)
                let val = self.add_high_low();

                // TODO 메모리 점프 (인터럽트)
                if val == 0 {
                    //제로디비전
                }

                let lhs_high = self.registers[14] as u32;
                let lhs_low = self.registers[15] as u32;
                let full_dividend = (lhs_high << 16) | lhs_low;

                let quotient = full_dividend / (val as u32);
                let remainder = full_dividend % (val as u32);

                // TODO 메모리 점프 (인터럽트)
                if quotient > 0xFFFF {
                    //디바이브 오버플로우
                }

                self.registers[15] = quotient as u16;
                self.registers[14] = remainder as u16;

                self.set_flag(crate::vm::ZF, quotient == 0);
                self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
            }

            0x23 => {
                // divr <Register>
                // [R14:R15](32비트) / <Register>(16비트) = R15(몫), R14(나머지)
                let reg = self.get_memory_u8();
                self.pc += 1;

                let val = self.registers[reg as usize];

                // TODO 메모리 점프 (인터럽트)
                if val == 0 {
                    // Divide by Zero Exception!
                }

                let lhs_high = self.registers[14] as u32;
                let lhs_low = self.registers[15] as u32;
                let full_dividend = (lhs_high << 16) | lhs_low;

                let quotient = full_dividend / (val as u32);
                let remainder = full_dividend % (val as u32);

                // TODO 메모리 점프 (인터럽트)
                if quotient > 0xFFFF {
                    //Divide Overflow Exception
                }

                self.registers[15] = quotient as u16;
                self.registers[14] = remainder as u16;

                self.set_flag(crate::vm::ZF, quotient == 0);
                self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
            }

            0x28 => self.binary_logic(|a, b| a & b),
            0x29 => self.binary_logic(|a, b| a | b),
            0x2A => self.binary_logic(|a, b| a ^ b),

            0x2B => self.immediate_logic(|a, b| a & b),
            0x2C => self.immediate_logic(|a, b| a | b),
            0x2D => self.immediate_logic(|a, b| a ^ b),

            0x2E => self.unary_logic(|a| !a),

            0x38 => {
                // jmp <LOWb> <HIGHb> (PC = Lb + Hb)
                let addr = self.add_high_low();
                self.pc = addr as usize;
            }
            0x39 => {
                // jcc <Condition> <LOWb> <HIGHb>

                let condition = self.get_memory_u8();
                self.pc += 1;

                let addr = self.add_high_low();

                let cond_result = match condition {
                    0x00 => true, // Unconditional
                    0x01 => self.get_flag(crate::vm::ZF),
                    0x02 => !self.get_flag(crate::vm::ZF),
                    0x03 => self.get_flag(crate::vm::SF),
                    0x04 => !self.get_flag(crate::vm::SF),
                    0x05 => self.get_flag(crate::vm::CF),
                    0x06 => !self.get_flag(crate::vm::CF),
                    0x07 => self.get_flag(crate::vm::OF),
                    0x08 => !self.get_flag(crate::vm::OF),
                    _ => {} // Unknown condition code
                };

                if cond_result {
                    self.pc = addr as usize;
                }
            }

            _ => {
                // Unknown opcode
            }
        }
    }
}
