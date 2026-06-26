use crate::vm::Vm;

impl Vm {
    // jmp <LOWb> <HIGHb>
    pub fn jmp(&mut self) {
        let addr = self.get_high_low();
        self.pc = addr as usize;
    }

    // je <LOWb> <HIGHb>
    pub fn je(&mut self) {
        let addr = self.get_high_low();
        if self.get_flag(crate::vm::ZF) {
            self.pc = addr as usize;
        }
    }

    // jne <LOWb> <HIGHb>
    pub fn jne(&mut self) {
        let addr = self.get_high_low();
        if !self.get_flag(crate::vm::ZF) {
            self.pc = addr as usize;
        }
    }

    // ja (unsigned >)
    pub fn ja(&mut self) {
        let addr = self.get_high_low();
        if !self.get_flag(crate::vm::CF) && !self.get_flag(crate::vm::ZF) {
            self.pc = addr as usize;
        }
    }

    // jae (unsigned >=)
    pub fn jae(&mut self) {
        let addr = self.get_high_low();
        if !self.get_flag(crate::vm::CF) {
            self.pc = addr as usize;
        }
    }

    // jb (unsigned <)
    pub fn jb(&mut self) {
        let addr = self.get_high_low();
        if self.get_flag(crate::vm::CF) {
            self.pc = addr as usize;
        }
    }

    // jbe (unsigned <=)
    pub fn jbe(&mut self) {
        let addr = self.get_high_low();
        if self.get_flag(crate::vm::CF) || self.get_flag(crate::vm::ZF) {
            self.pc = addr as usize;
        }
    }

    // jg (signed >)
    pub fn jg(&mut self) {
        let addr = self.get_high_low();

        let sf = self.get_flag(crate::vm::SF);
        let of = self.get_flag(crate::vm::OF);

        if !self.get_flag(crate::vm::ZF) && (sf == of) {
            self.pc = addr as usize;
        }
    }

    // jge (signed >=)
    pub fn jge(&mut self) {
        let addr = self.get_high_low();

        let sf = self.get_flag(crate::vm::SF);
        let of = self.get_flag(crate::vm::OF);

        if sf == of {
            self.pc = addr as usize;
        }
    }

    // jl (signed <)
    pub fn jl(&mut self) {
        let addr = self.get_high_low();

        let sf = self.get_flag(crate::vm::SF);
        let of = self.get_flag(crate::vm::OF);

        if sf != of {
            self.pc = addr as usize;
        }
    }

    // jle (signed <=)
    pub fn jle(&mut self) {
        let addr = self.get_high_low();

        let sf = self.get_flag(crate::vm::SF);
        let of = self.get_flag(crate::vm::OF);

        if self.get_flag(crate::vm::ZF) || (sf != of) {
            self.pc = addr as usize;
        }
    }
}
