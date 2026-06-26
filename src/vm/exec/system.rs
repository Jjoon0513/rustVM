use crate::vm::exec::interrupt::Interrupt;
use crate::vm::{IF, REG_RET_FLAGS, REG_RET_PC, Vm};

impl Vm {
    pub fn syscall(&mut self) {
        if self.cpl == 0 {
            return; // 아 몰라 os에서 안하겠지머
        }

        self.registers[REG_RET_PC] = self.pc as u16;
        self.registers[REG_RET_FLAGS] = self.flags as u16;

        self.cpl = 0;
        self.pc = self.lstar;
    }

    pub fn sysret(&mut self) {
        if self.cpl != 0 {
            self.interrupt(Interrupt::GeneralProtection as u8);
            return;
        }

        self.pc = self.registers[REG_RET_PC] as usize;
        self.flags = self.registers[REG_RET_FLAGS] as u8;

        self.cpl = 3;
    }

    pub fn hlt(&mut self) {
        if self.cpl != 0 {
            self.interrupt(Interrupt::GeneralProtection as u8);
            return;
        }

        self.halt = true;
    }

    pub fn cli(&mut self) {
        if self.cpl != 0 {
            self.interrupt(Interrupt::GeneralProtection as u8);
            return;
        }

        self.set_flag(IF, false);
    }

    pub fn sti(&mut self) {
        if self.cpl != 0 {
            self.interrupt(Interrupt::GeneralProtection as u8);
            return;
        }

        self.set_flag(IF, true);
    }
}
