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
0x00~0x0F  시스템
0x10~0x1F  MOV 계열
0x20~0x2F  ADD 계열
0x30~0x3F  SUB 계열
0x40~0x4F  JMP 계열
0xF0~0xFF  특수 명령
         */

        match opcode {
            0x00 => {} //nop
            0x10 => {
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
            0x11 => {
                //movr: movr <Register0> <Register1> (R1 -> R0)
                //TODO
            }
            _ => panic!("Unknown opcode"),
        }
    }
}