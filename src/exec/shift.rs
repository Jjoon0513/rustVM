use crate::vm::Vm;

impl Vm {
    // shli <Register> <Amount>
    // Register <<= immediate
    pub fn shli(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let amount = self.fetch_u8();
        self.pc += 1;

        let value = self.registers[reg as usize];
        let result = value << amount;

        self.registers[reg as usize] = result;

        // 마지막으로 밀려나간 bit
        let carry = if amount == 0 {
            false
        } else if amount <= 16 {
            (value & (1 << (16 - amount))) != 0
        } else {
            false
        };

        self.set_flag(crate::vm::CF, carry);

        let old_msb = (value & 0x8000) != 0;
        let new_msb = (result & 0x8000) != 0;

        self.set_flag(crate::vm::OF, old_msb != new_msb);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, (result & 0x8000) != 0);
    }

    // shlr <DstRegister> <SrcRegister>
    // DstRegister <<= SrcRegister
    pub fn shlr(&mut self) {
        let dst = self.fetch_u8();
        self.pc += 1;

        let src = self.fetch_u8();
        self.pc += 1;

        let amount = self.registers[src as usize] as u8;

        let value = self.registers[dst as usize];
        let result = value << amount;

        self.registers[dst as usize] = result;

        let carry = if amount == 0 {
            false
        } else if amount <= 16 {
            (value & (1 << (16 - amount))) != 0
        } else {
            false
        };

        self.set_flag(crate::vm::CF, carry);

        let old_msb = (value & 0x8000) != 0;
        let new_msb = (result & 0x8000) != 0;

        self.set_flag(crate::vm::OF, old_msb != new_msb);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, (result & 0x8000) != 0);
    }

    // shri <Register> <Amount>
    // Register >>= immediate
    pub fn shri(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let amount = self.fetch_u8();
        self.pc += 1;

        let value = self.registers[reg as usize];
        let result = value >> amount;

        self.registers[reg as usize] = result;

        // 마지막으로 밀려나간 bit
        let carry = if amount == 0 {
            false
        } else if amount <= 16 {
            (value & (1 << (amount - 1))) != 0
        } else {
            false
        };

        self.set_flag(crate::vm::CF, carry);

        // logical shift right는 overflow 없음
        self.set_flag(crate::vm::OF, false);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, (result & 0x8000) != 0);
    }

    // shrr <DstRegister> <SrcRegister>
    // DstRegister >>= SrcRegister
    pub fn shrr(&mut self) {
        let dst = self.fetch_u8();
        self.pc += 1;

        let src = self.fetch_u8();
        self.pc += 1;

        let amount = self.registers[src as usize] as u8;

        let value = self.registers[dst as usize];
        let result = value >> amount;

        self.registers[dst as usize] = result;

        let carry = if amount == 0 {
            false
        } else if amount <= 16 {
            (value & (1 << (amount - 1))) != 0
        } else {
            false
        };

        self.set_flag(crate::vm::CF, carry);

        self.set_flag(crate::vm::OF, false);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, (result & 0x8000) != 0);
    }
}
