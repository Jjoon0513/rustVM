const CF: u8 = 1 << 0;
const PF: u8 = 1 << 1;
const AF: u8 = 1 << 2;
const ZF: u8 = 1 << 3;
const SF: u8 = 1 << 4;
const OF: u8 = 1 << 5;

pub struct Vm {
    pub registers: [u16; 16],
    pub pc: usize,
    pub memory: [u8; 65536],
    pub flags: u8,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
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

    pub fn get_memory_u16(&self) -> u16 {
        self.memory[self.pc as usize] as u16
    }



    ///이 함수는 프로그램 카운터를 두칸 앞으로 옮김 `self.pc += 2`
    ///
    /// Advances the program counter by two. `self.pc += 2`
    pub fn add_high_low(&mut self) -> u16 {
        let low= self.get_memory_u16();
        self.pc += 1;

        let high = self.get_memory_u16();
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

            /*
            TODO
            MULI, MULR (0x20, 0x21)
            그리고 저 연산함수에서 레지스트리가 u16 오버플로면 오버플로 플래그도 옮기게 바꿔야함 :)
             */

            _ => panic!("Unknown opcode"),
        }

    }
}
