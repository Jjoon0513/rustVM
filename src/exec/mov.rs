use crate::vm::Vm;

impl Vm {
    //movi: movi <Register> <LOWb> <HIGHb> (Lb + Hb -> R)c
    pub fn movi(&mut self) {
        let reg = self.get_memory_u8();
        self.pc += 1;

        let value = self.add_high_low();

        self.registers[reg as usize] = value;
    }
    //movr: movr <Register0> <Register1> (R1 -> R0)
    pub fn movr(&mut self) {
        let reg0 = self.get_memory_u8();
        self.pc += 1;

        let reg1 = self.get_memory_u8();
        self.pc += 1;

        self.registers[reg0 as usize] = self.registers[reg1 as usize];
    }
}
