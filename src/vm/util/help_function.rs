use crate::vm::{Vm, REG_SP, REG_ZR};

impl Vm {
    pub fn set_register(&mut self, reg: usize, value: u16) {
        if reg == REG_SP {
            if self.cpl == 0 {
                self.ksp = value as usize;
            } else {
                self.usp = value as usize;
            }
            return;
        }
        if reg != REG_ZR {
            self.registers[reg] = value;
        }
    }

    pub fn get_register(&self, reg: usize) -> u16 {
        if reg == REG_SP {
            if self.cpl == 0 {
                return self.ksp as u16;
            } else {
                return self.usp as u16;
            }
        }
        // if reg == REG_ZR {
        //     return 0;
        // }
        // 사실 이렇게 해도 되는데 또 분기생기면 성능 손해볼꺼같음...
        self.registers[reg]
    }
}

impl Vm {
    pub fn binary_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg0 = self.fetch_u8();
        self.pc += 1;

        let reg1 = self.fetch_u8();
        self.pc += 1;

        let rst = op(self.registers[reg0 as usize], self.registers[reg1 as usize]);

        self.registers[reg0 as usize] = rst;
        self.update_flags(rst);
    }

    pub fn update_flags(&mut self, rst: u16) {
        self.set_flag(crate::vm::CF, false);
        self.set_flag(crate::vm::OF, false);
        self.set_flag(crate::vm::ZF, rst == 0);
        self.set_flag(crate::vm::SF, rst & 0x8000 != 0);
    }

    pub fn unary_logic<F>(&mut self, op: F)
    where
        F: Fn(u16) -> u16,
    {
        let reg = self.fetch_u8();
        if reg == REG_ZR as u8 {
            self.pc += 1;
            return;
        }
        self.pc += 1;

        let rst = op(self.registers[reg as usize]);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
    pub fn immediate_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg = self.fetch_u8();
        if reg == REG_ZR as u8 {
            self.pc += 1;
            return;
        }
        self.pc += 1;

        let val = self.get_high_low();

        let rst = op(self.registers[reg as usize], val);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
}
