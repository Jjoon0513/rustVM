use crate::vm::Vm;

impl Vm {
    pub fn binary_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg0 = self.fetch_u8();
        self.pc += 1;

        let reg1 = self.fetch_u8();
        self.pc += 1;

        let rst = op(self.registers[reg0 as usize], self.registers[reg1 as usize]);

        self.registers[reg0 as usize] = rst;
        self.update_flags(rst);
    }

    pub fn update_flags(&mut self, rst: u16) {
        self.set_flag(crate::vm::CF, false);
        self.set_flag(crate::vm::OF, false);
        self.set_flag(crate::vm::ZF, rst == 0);
        self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
    }

    pub fn unary_logic<F>(&mut self, op: F)
    where
        F: Fn(u16) -> u16,
    {
        let reg = self.fetch_u8();
        self.pc += 1;

        let rst = op(self.registers[reg as usize]);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
    pub fn immediate_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg = self.fetch_u8();
        self.pc += 1;

        let val = self.get_high_low();

        let rst = op(self.registers[reg as usize], val);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
}
