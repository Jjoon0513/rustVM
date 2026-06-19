use crate::vm::Vm;

// muli <LOWb> + <HIGHb> * <R15> = <R14(HIGH)> , <R15(LOW)>
impl Vm {
    pub fn muli(&mut self) {
        let rhs = self.get_high_low();
        let lhs = self.registers[15];

        let product = (lhs as u32) * (rhs as u32);

        self.registers[14] = (product >> 16) as u16;
        self.registers[15] = product as u16;

        self.set_flag(crate::vm::CF, product > 0xFFFF);
        self.set_flag(crate::vm::OF, product > 0xFFFF);
        self.set_flag(crate::vm::ZF, product == 0);
        self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
    }

    // mulr <Register> * <R15> = <R14(HIGH)> , <R15(LOW)>
    pub fn mulr(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let lhs = self.registers[15];
        let rhs = self.registers[reg as usize];

        let product = (lhs as u32) * (rhs as u32);

        self.registers[14] = (product >> 16) as u16;
        self.registers[15] = product as u16;

        self.set_flag(crate::vm::CF, product > 0xFFFF);
        self.set_flag(crate::vm::OF, product > 0xFFFF);
        self.set_flag(crate::vm::ZF, product == 0);
        self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
    }

    // divi <LOWb> + <HIGHb>
    // [R14:R15](32bit) / immediate = R15(quotient), R14(remainder)
    pub fn divi(&mut self) {
        let rhs = self.get_high_low();

        // TODO interrupt
        if rhs == 0 {
            // Divide by Zero Exception
        }

        let high = self.registers[14] as u32;
        let low = self.registers[15] as u32;

        let dividend = (high << 16) | low;

        let quotient = dividend / (rhs as u32);
        let remainder = dividend % (rhs as u32);

        // TODO interrupt
        if quotient > 0xFFFF {
            // Divide Overflow Exception
        }

        self.registers[15] = quotient as u16;
        self.registers[14] = remainder as u16;

        self.set_flag(crate::vm::ZF, quotient == 0);
        self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
    }

    // divr <Register>
    // [R14:R15](32bit) / register = R15(quotient), R14(remainder)
    pub fn divr(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let rhs = self.registers[reg as usize];

        // TODO interrupt
        if rhs == 0 {
            // Divide by Zero Exception
        }

        let high = self.registers[14] as u32;
        let low = self.registers[15] as u32;

        let dividend = (high << 16) | low;

        let quotient = dividend / (rhs as u32);
        let remainder = dividend % (rhs as u32);

        // TODO interrupt
        if quotient > 0xFFFF {
            // Divide Overflow Exception
        }

        self.registers[15] = quotient as u16;
        self.registers[14] = remainder as u16;

        self.set_flag(crate::vm::ZF, quotient == 0);
        self.set_flag(crate::vm::SF, (self.registers[15] & 0x8000) != 0);
    }
}
