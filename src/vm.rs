const CF: u8 = 1 << 0;
const PF: u8 = 1 << 1;
const AF: u8 = 1 << 2;
const ZF: u8 = 1 << 3;
const SF: u8 = 1 << 4;
const OF: u8 = 1 << 5;

pub struct Vm {
    registers: [u16; 8],
    pc: u16,
    memory: [u8; 65536],
    flags: u8,
}

impl Vm {
    fn new() -> Self {
        Self {
            registers: [0; 8],
            pc: 0,
            memory: [0; 65536],
            flags: 0,
        }
    }
    fn step(&mut self) {
        let opcode = self.memory[self.pc as usize];
        self.pc += 1;

        /*
0x00 ~ 0x07	시스템
0x08 ~ 0x0F	MOV
0x10 ~ 0x17	ADD
0x18 ~ 0x1F	SUB
0x20 ~ 0x27	AND
0x28 ~ 0x2F	OR/XOR
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
                let reg = self.memory[self.pc as usize];
                self.pc += 1;

                let low = self.memory[self.pc as usize] as u16;
                self.pc += 1;

                let high = self.memory[self.pc as usize] as u16;
                self.pc += 1;

                let value = low | (high << 8);

                self.registers[reg as usize] = value;
            }
            0x09 => {

                //movr: movr <Register0> <Register1> (R1 -> R0)
                let reg0 = self.memory[self.pc as usize];
                self.pc += 1;

                let reg1 = self.memory[self.pc as usize];
                self.pc += 1;

                self.registers[reg0 as usize] = self.registers[reg1 as usize];
            }

            0x

            _ => panic!("Unknown opcode"),
        }

    }
}