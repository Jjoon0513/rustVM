mod exec;
mod step;
mod util;

pub const CF: u8 = 1 << 0;
pub const ZF: u8 = 1 << 3;
pub const SF: u8 = 1 << 4;
pub const OF: u8 = 1 << 5;
pub const IF: u8 = 1 << 6;

pub const REG_RET_PC: usize = 14;
pub const REG_RET_FLAGS: usize = 15;

pub const REG_ZR: usize = 16;
pub const REG_SP: usize = 17;

pub const DEFAULT_USER_STACK_ADDR: usize = 0xBFFF;
pub const DEFAULT_KERNEL_STACK_ADDR: usize = 0xFFFF;
pub const DEFAULT_PC_ADDR: usize = 0xC100;

pub struct Vm {
    // 레지스터
    pub registers: [u16; 18],
    /*
    R0 ~ R11 (범용)
    R12, R13 (DIV/MUL전용)
    R14 = REG_RET_PC
    R15 = REG_RET_FLAGS
    R16 = ZR (Zero Register, 항상 0)
    R17 = SP (Stack Pointer)
    */

    // MSR
    pub lstar: usize,
    pub cpl: u8,

    pub testmode: bool,
    pub usp: usize,
    pub ksp: usize,

    pub pc: usize,
    pub memory: Box<[u8; 65536]>,
    pub flags: u8,

    pub timer_ticks: u64,

    pub halt: bool,
}

/// # Use Reset() Function Before Using Vm!
///
impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0x00; 18],
            lstar: 0,
            cpl: 0,
            testmode: false,
            usp: DEFAULT_USER_STACK_ADDR,
            ksp: DEFAULT_KERNEL_STACK_ADDR,
            pc: DEFAULT_PC_ADDR,
            memory: Box::new([0; 65536]),
            flags: 0b00000000,
            timer_ticks: 0,
            halt: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
} //엄청난 하드코딩이다..!
