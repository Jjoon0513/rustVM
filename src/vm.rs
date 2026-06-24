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
