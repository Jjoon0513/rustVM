use crate::vm::Vm;
use crate::vm::exec::interrupt::Interrupt;

// muli <LOWb> + <HIGHb> * <R13> = <R12(HIGH)> , <R13(LOW)>
impl Vm {
    pub fn muli(&mut self) {
        let rhs = self.get_high_low();
        let lhs = self.registers[13];

        let product = (lhs as u32) * (rhs as u32);

        self.registers[12] = (product >> 16) as u16;
        self.registers[13] = product as u16;

        self.set_flag(crate::vm::CF, product > 0xFFFF);
        self.set_flag(crate::vm::OF, product > 0xFFFF);
        self.set_flag(crate::vm::ZF, product == 0);
        self.set_flag(crate::vm::SF, (self.registers[13] & 0x8000) != 0);
    }

    // mulr <Register> * <R13> = <R12(HIGH)> , <R13(LOW)>
    pub fn mulr(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let lhs = self.registers[13];
        let rhs = self.registers[reg as usize];

        let product = (lhs as u32) * (rhs as u32);

        self.registers[12] = (product >> 16) as u16;
        self.registers[13] = product as u16;

        self.set_flag(crate::vm::CF, product > 0xFFFF);
        self.set_flag(crate::vm::OF, product > 0xFFFF);
        self.set_flag(crate::vm::ZF, product == 0);
        self.set_flag(crate::vm::SF, (self.registers[13] & 0x8000) != 0);
    }

    // divi <LOWb> + <HIGHb>
    // [R12:R13](32bit) / immediate = R13(quotient), R12(remainder)
    pub fn divi(&mut self) {
        let rhs = self.get_high_low();

        if rhs == 0 {
            self.interrupt(Interrupt::DivideByZero as u8)
        }

        let high = self.registers[12] as u32;
        let low = self.registers[13] as u32;

        let dividend = (high << 16) | low;

        let quotient = dividend / (rhs as u32);
        let remainder = dividend % (rhs as u32);

        if quotient > 0xFFFF {
            self.interrupt(Interrupt::DivideByZero as u8)
        }

        self.registers[13] = quotient as u16;
        self.registers[12] = remainder as u16;

        self.set_flag(crate::vm::ZF, quotient == 0);
        self.set_flag(crate::vm::SF, (self.registers[13] & 0x8000) != 0);
    }

    // divr <Register>
    // [R12:R13](32bit) / register = R13(quotient), R12(remainder)
    pub fn divr(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let rhs = self.registers[reg as usize];

        if rhs == 0 {
            self.interrupt(Interrupt::DivideByZero as u8)
        }

        let high = self.registers[12] as u32;
        let low = self.registers[13] as u32;

        let dividend = (high << 16) | low;

        let quotient = dividend / (rhs as u32);
        let remainder = dividend % (rhs as u32);

        if quotient > 0xFFFF {
            self.interrupt(Interrupt::DivideByZero as u8)
        }

        self.registers[13] = quotient as u16;
        self.registers[12] = remainder as u16;

        self.set_flag(crate::vm::ZF, quotient == 0);
        self.set_flag(crate::vm::SF, (self.registers[13] & 0x8000) != 0);
    }
}
