use crate::vm::Vm;

#[repr(u8)]
pub enum Interrupt {
    DivideByZero = 0x00,      // #DE
    Debug = 0x01,             // #DB
    InvalidOpcode = 0x06,     // #UD
    GeneralProtection = 0x0D, // #GP
    PageFault = 0x0E,         // #PF
    InvalidRegister = 0x10,   // #NR

    //0x20 ~ 0x3F 는 maskable interrupt vector로 예약되어 있음
    Timer = 0x20,
    Keyboard = 0x21,

    SyscallLegacy = 0x80,
}
impl Vm {
    pub fn interrupt_op(&mut self) {
        let int_num = self.fetch_u8();
        self.pc += 1;
        self.interrupt(int_num);
    }
}
