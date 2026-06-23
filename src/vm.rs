use crate::exec::interrupt::Interrupt;

pub const CF: u8 = 1 << 0;
pub const ZF: u8 = 1 << 3;
pub const SF: u8 = 1 << 4;
pub const OF: u8 = 1 << 5;
pub const IF: u8 = 1 << 6;

pub const REG_RET_PC: usize = 14;
pub const REG_RET_FLAGS: usize = 15;

/*
R0 ~ R13 (범용)
R14, R15 (DIV/MUL전용 (그리고 범용))
*/
pub struct Vm {
    // 레지스터
    pub registers: [u16; 16],
    // MSR
    pub lstar: usize,
    pub cpl: u8,
    pub kernel_gs_base: u16,

    pub usp: usize,
    pub ksp: usize,

    pub pc: usize,
    pub memory: [u8; 65536],
    pub flags: u8,

    pub timer_ticks: u64,

    pub halt: bool,
}

// ::new
impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0x00; 16],
            lstar: 0,
            cpl: 0,
            kernel_gs_base: 0, //지금은 DEAD?

            usp: 0xBFFF,
            ksp: 0xFFFF,

            pc: 0xC100,
            memory: [0; 65536],
            flags: 0b00000000,

            timer_ticks: 0,

            halt: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }

    pub fn run_max(&mut self, max_steps: u64) -> bool {
        let mut steps = 0;
        while !self.halt && steps < max_steps {
            self.step();
            steps += 1;
        }
        self.halt // true = 정상 hlt, false = 스텝 초과
    }
} //엄청난 하드코딩이다..!

// 핼퍼함수 모음
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
        self.pc += 1;

        let val = self.get_high_low();

        let rst = op(self.registers[reg as usize], val);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
}

//  0x0000 ~ 0x00FF       |    256 B     |   Kernel   | 인터럽트/시스콜 벡터 테이블 (MSR 대용)
//  0x0100 ~ 0x9FFF       |   40.7 KB    |    User    | 유저 프로그램 코드 & 데이터 (텍스트, 힙)
//  0xA000 ~ 0xBFFF       |     8 KB     |    User    | 유저 스택 공간 (0xBFFF부터 아래로 감소)
//  0xC000 ~ 0xC0FF       |    256 B     |   Kernel   | MMIO 장치 구역 (하드웨어 I/O 레지스터)
//  0xC100 ~ 0xF000       |   11.7 KB    |   Kernel   | 게스트 커널 소스 코드 및 드라이버
//  0xF001 ~ 0xFFFF       |     4 KB     |   Kernel   | 커널 전용 스택 공간 (0xFFFF부터 아래로 감소)

//  MMIO
//  0xC000 : [WRITE] UART TX
//  0xC001 : [READ]  UART RX
//  0xC002 : [READ]  TIMER
//  0xC003 : [READ]  RANDOM

//유틸
impl Vm {
    pub fn interrupt(&mut self, int_num: u8) {
        self.halt = false;
        if !self.get_flag(IF) && (0x20..=0x3F).contains(&int_num) {
            return;
        }

        let handler = self.memory[(int_num * 2) as usize] as u16
            | ((self.memory[(int_num * 2 + 1) as usize] as u16) << 8);
        self.push_kernel_stack(self.flags);
        self.push_kernel_stack((self.pc >> 8) as u8);
        self.push_kernel_stack((self.pc & 0xFF) as u8);
        self.push_kernel_stack(self.cpl);
        self.cpl = 0;
        self.pc = handler as usize;
    }

    pub fn iret(&mut self) {
        if self.cpl != 0 {
            self.interrupt(Interrupt::GeneralProtection as u8);
            return;
        }

        let old_cpl = self.pop_kernel_stack();

        let pc_low = self.pop_kernel_stack() as u16;
        let pc_high = self.pop_kernel_stack() as u16;

        let old_flags = self.pop_kernel_stack();

        self.cpl = old_cpl;
        self.pc = ((pc_high << 8) | pc_low) as usize;
        self.flags = old_flags;
    }

    pub fn split_u16_as_u8(&mut self, value: u16) -> (u8, u8) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        (low, high)
    }
    pub fn combine_u8_to_u16(&mut self, low: u8, high: u8) -> u16 {
        low as u16 | ((high as u16) << 8)
    }
    pub fn push_kernel_stack(&mut self, data: u8) {
        self.ksp -= 1;
        self.set_memory(data, self.ksp);
    }

    pub fn push_user_stack(&mut self, data: u8) {
        self.usp -= 1;
        self.set_memory(data, self.usp);
    }

    pub fn pop_kernel_stack(&mut self) -> u8 {
        let data = self.memory[self.ksp];
        self.ksp += 1;
        data
    }

    pub fn pop_user_stack(&mut self) -> u8 {
        let data = self.memory[self.usp];
        self.usp += 1;
        data
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn set_memory(&mut self, data: u8, ptr: usize) {
        self.memory[ptr] = data
    }

    pub fn get_memory(&self, ptr: usize) -> u8 {
        self.memory[ptr as usize]
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    pub fn fetch_u8(&self) -> u8 {
        self.get_memory(self.pc)
    }

    pub fn fetch_u16(&self) -> u16 {
        self.get_memory(self.pc) as u16
    }

    ///이 함수는 프로그램 카운터를 두칸 앞으로 옮김 `self.pc += 2`
    ///
    /// Advances the program counter by two. `self.pc += 2`
    pub fn get_high_low(&mut self) -> u16 {
        let low = self.fetch_u16();
        self.pc += 1;

        let high = self.fetch_u16();
        self.pc += 1;

        low | (high << 8)
    }
}
