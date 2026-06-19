use crate::vm::Vm;

impl Vm{
    pub fn push(&mut self) {
        let reg = self.fetch_u8();
        let (low, high) = self.split_u16(self.registers[reg as usize]);
        if self.cpl == 0{
            self.push_kernel_stack(high);
            self.push_kernel_stack(low);
        } 
        else {
            self.push_user_stack(high);
            self.push_user_stack(low);
        }
        
    }

    // pub fn pop(&mut self) TODO

}