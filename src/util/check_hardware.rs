use crate::exec::interrupt::Interrupt;
use crate::vm::Vm;

impl Vm {
    pub fn check_hardware(&mut self) {

        

        self.timer_ticks += 1;

        if self.timer_ticks >= 100 {
            self.timer_ticks = 0;
            self.interrupt(Interrupt::Timer as u8);
        }
    }
}
