use crate::vm::Vm;

impl Vm {
    // loadr: load <dst_reg> <addr_reg>
    pub fn loadr(&mut self) {
        let dst = self.fetch_u8();
        self.pc += 1;
        let addr_reg = self.fetch_u8();
        self.pc += 1;

        let addr = self.registers[addr_reg as usize] as usize;
        let low = self.get_memory(addr);
        let high = self.get_memory(addr + 1);
        self.registers[dst as usize] = self.combine_u8_to_u16(low, high);
    }

    // storer: store <addr_reg> <src_reg>
    pub fn storer(&mut self) {
        let addr_reg = self.fetch_u8();
        self.pc += 1;
        let src = self.fetch_u8();
        self.pc += 1;

        let addr = self.registers[addr_reg as usize] as usize;
        let (low, high) = self.split_u16_as_u8(self.registers[src as usize]);
        self.set_memory(low, addr);
        self.set_memory(high, addr + 1);
    }
    pub fn loadi(&mut self) {
        let dst = self.fetch_u8();
        self.pc += 1;

        let addr = self.get_high_low();

        let low = self.get_memory(addr as usize);
        let high = self.get_memory(addr as usize + 1);

        self.registers[dst as usize] = self.combine_u8_to_u16(low, high);
    }

    pub fn storei(&mut self) {
        let addr = self.get_high_low();

        let src = self.fetch_u8();
        self.pc += 1;

        let (low, high) = self.split_u16_as_u8(self.registers[src as usize]);

        self.set_memory(low, addr as usize);
        self.set_memory(high, addr as usize + 1);
    }
}
