use crate::vm::Vm;

impl Vm {
    pub fn call(&mut self) {
        let addr = self.get_high_low();

        let return_addr = self.pc;
        let (low, high) = self.split_u16_as_u8(return_addr as u16);
        if self.cpl == 0 {
            self.push_kernel_stack(high);
            self.push_kernel_stack(low);
            self.push_kernel_stack(self.cpl);
        } else {
            self.push_user_stack(high);
            self.push_user_stack(low);
            self.push_user_stack(self.cpl);
        }

        self.pc = addr as usize;
    }

    pub fn ret(&mut self) {
        if self.cpl == 0 {
            self.cpl = self.pop_kernel_stack();
            let low = self.pop_kernel_stack();
            let high = self.pop_kernel_stack();
            self.pc = self.combine_u8_to_u16(low, high) as usize;
        } else {
            self.cpl = self.pop_user_stack();
            let low = self.pop_user_stack();
            let high = self.pop_user_stack();
            self.pc = self.combine_u8_to_u16(low, high) as usize;
        }
    }
}

