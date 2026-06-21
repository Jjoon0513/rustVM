use crate::exec::interrupt::Interrupt;
use crate::vm::Vm;

impl Vm {
    pub fn step(&mut self) {
        self.check_hardware();


        if self.halt {
            return;
        }


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
        0x60 ~ 0x67  SHIFT
        0x68 ~ ...  ETC
        */

        match opcode {
            0x00 => {} //nop
            0x01 => self.syscall(),
            0x02 => self.sysret(),
            0x03 => self.interrupt_op(),
            0x04 => self.hlt(),
            0x05 => self.cli(),
            0x06 => self.sti(),
            0x07 => self.iret(),

            0x08 => self.movi(),
            0x09 => self.movr(),

            0x18 => self.addi(),
            0x19 => self.addr(),

            0x1A => self.subi(),
            0x1B => self.subr(),
            0x1C => self.cmp(),

            // MULI, MULR, DIVI, DIVR은 R12, R13에 결과값을 저장함 (syscall에 양보하기로 했음)
            0x20 => self.muli(),
            0x21 => self.mulr(), //아니이런세상멍청한실수를하다니.
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

            0x40 => self.push(),
            0x41 => self.pop(),

            0x48 => self.call(),
            0x49 => self.ret(),

            0x50 => self.loadr(),
            0x51 => self.loadi(),
            0x52 => self.storer(),
            0x53 => self.storei(),

            0x60 => self.shli(),
            0x61 => self.shlr(),
            0x62 => self.shri(),
            0x63 => self.shrr(),

            _ => {
                self.interrupt(Interrupt::InvalidOpcode as u8);
            }
        }
    }
}
