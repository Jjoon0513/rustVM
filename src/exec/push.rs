use crate::vm::Vm;

impl Vm {
    pub fn push(&mut self) {
        let reg = self.fetch_u8();
        let (low, high) = self.split_u16_as_u8(self.registers[reg as usize]);
        if self.cpl == 0 {
            self.push_kernel_stack(high);
            self.push_kernel_stack(low);
        } else {
            self.push_user_stack(high);
            self.push_user_stack(low);
        }
    }

    pub fn pop(&mut self) {
        let reg = self.fetch_u8();

        let (low, high) = if self.cpl == 0 {
            (self.pop_kernel_stack(), self.pop_kernel_stack())
        } else {
            (self.pop_user_stack(), self.pop_user_stack())
        };

        self.registers[reg as usize] = self.combine_u8_to_u16(low, high);
    }
}
