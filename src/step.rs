use crate::exec::*;
use crate::vm::Vm;

impl Vm {
    pub fn step(&mut self) {
        let opcode = self.fetch_u8();
        self.pc += 1;

        /*
        0x00 ~ 0x07	시스템
        0x08 ~ 0x0F	MOV
        0x10 ~ 0x17	Undecided
        0x18 ~ 0x1F	ADD/SUB
        0x20 ~ 0x27	MUL/DIV
        0x28 ~ 0x2F	AND/OR/XOR
        0x30 ~ 0x37	JMP
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

            0x18 => self.addi(),
            0x19 => self.addr(),

            0x1A => self.subi(),
            0x1B => self.subr(),
            0x1C => self.cmp(),

            // MULI, MULR, DIVI, DIVR은 특수레지스트리 R15를 주로 연산하고 R14, R15에 결과값을 저장함
            0x20 => self.muli(),

            0x21 => self.movr(),
            0x22 => self.divi(),
            0x23 => self.divr(),

            0x28 => self.binary_logic(|a, b| a & b),
            0x29 => self.binary_logic(|a, b| a | b),
            0x2A => self.binary_logic(|a, b| a ^ b),

            0x2B => self.immediate_logic(|a, b| a & b),
            0x2C => self.immediate_logic(|a, b| a | b),
            0x2D => self.immediate_logic(|a, b| a ^ b),

            0x2E => self.unary_logic(|a| !a),

            0x30 => self.jmp(),
            0x31 => self.je(),
            0x32 => self.jne(),

            0x33 => self.ja(),
            0x34 => self.jae(),
            0x35 => self.jb(),
            0x36 => self.jbe(),

            0x37 => self.jg(),
            0x38 => self.jge(),
            0x39 => self.jl(),
            0x3A => self.jle(),

            _ => {
                // Unknown opcode
            }
        }
    }
}
