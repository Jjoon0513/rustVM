use crate::vm::Vm;

impl Vm {
    // addi: addi <Register> <LOWb> <HIGHb> (R += Lb + Hb)
    pub fn addi(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let imm = self.get_high_low();
        let lhs = self.registers[reg as usize];

        let (result, carry) = lhs.overflowing_add(imm);

        self.set_flag(crate::vm::CF, carry);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, result & 0x8000 != 0);
        self.set_flag(
            crate::vm::OF,
            ((lhs ^ result) & (imm ^ result) & 0x8000) != 0,
        );

        self.registers[reg as usize] = result;
    }

    // addr: addr <Register0> <Register1> (R0 += R1)
    pub fn addr(&mut self) {
        let reg0 = self.fetch_u8();
        self.pc += 1;

        let reg1 = self.fetch_u8();
        self.pc += 1;

        let lhs = self.registers[reg0 as usize];
        let rhs = self.registers[reg1 as usize];

        let (result, carry) = lhs.overflowing_add(rhs);

        self.set_flag(crate::vm::CF, carry);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, result & 0x8000 != 0);
        self.set_flag(
            crate::vm::OF,
            ((lhs ^ result) & (rhs ^ result) & 0x8000) != 0,
        );

        self.registers[reg0 as usize] = result;
    }

    // subi: subi <Register> <LOWb> <HIGHb> (R -= Lb + Hb)
    pub fn subi(&mut self) {
        let reg = self.fetch_u8();
        self.pc += 1;

        let imm = self.get_high_low();
        let lhs = self.registers[reg as usize];

        let (result, borrow) = lhs.overflowing_sub(imm);

        self.set_flag(crate::vm::CF, borrow);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, result & 0x8000 != 0);
        self.set_flag(crate::vm::OF, ((lhs ^ imm) & (lhs ^ result) & 0x8000) != 0);

        self.registers[reg as usize] = result;
    }

    // subr: subr <Register0> <Register1> (R0 -= R1)
    pub fn subr(&mut self) {
        let reg0 = self.fetch_u8();
        self.pc += 1;

        let reg1 = self.fetch_u8();
        self.pc += 1;

        let lhs = self.registers[reg0 as usize];
        let rhs = self.registers[reg1 as usize];

        let (result, borrow) = lhs.overflowing_sub(rhs);

        self.set_flag(crate::vm::CF, borrow);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, result & 0x8000 != 0);
        self.set_flag(crate::vm::OF, ((lhs ^ rhs) & (lhs ^ result) & 0x8000) != 0);

        self.registers[reg0 as usize] = result;
    }

    // cmp: cmp <Register0> <Register1> (R0 - R1)
    pub fn cmp(&mut self) {
        let reg0 = self.fetch_u8();
        self.pc += 1;

        let reg1 = self.fetch_u8();
        self.pc += 1;

        let lhs = self.registers[reg0 as usize];
        let rhs = self.registers[reg1 as usize];

        let (result, borrow) = lhs.overflowing_sub(rhs);

        self.set_flag(crate::vm::CF, borrow);
        self.set_flag(crate::vm::ZF, result == 0);
        self.set_flag(crate::vm::SF, result & 0x8000 != 0);
        self.set_flag(crate::vm::OF, ((lhs ^ rhs) & (lhs ^ result) & 0x8000) != 0);
    }
}
