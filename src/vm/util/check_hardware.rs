use crate::vm::Vm;
use crate::vm::exec::interrupt::Interrupt;

impl Vm {
    pub fn check_hardware(&mut self) {
        self.timer_ticks += 1;

        if self.timer_ticks >= 100 {
            self.timer_ticks = 0;
            self.interrupt(Interrupt::Timer as u8);
        }
    }
    pub fn uart_write(&mut self, value: u8) {
        print!("{}", value as char);
    }
}
