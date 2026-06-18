pub const CF: u8 = 1 << 0;
pub const PF: u8 = 1 << 1;
pub const AF: u8 = 1 << 2;
pub const ZF: u8 = 1 << 3;
pub const SF: u8 = 1 << 4;
pub const OF: u8 = 1 << 5;

//TODO
/*
R0 ~ R13 (범용)
R14, R15 (DIV/MUL전용)
*/

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
pub struct Vm {
    // 레지스터
    pub registers: [u16; 16],
    // MSR
    pub lstar: u16,
    pub cpl: u8,
    pub kernel_gs_base: u16,

    pub pc: usize,
    pub memory: [u8; 65536],
    pub flags: u8,
}

impl Vm {
    pub fn binary_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg0 = self.get_memory_u8();
        self.pc += 1;

        let reg1 = self.get_memory_u8();
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
        let reg = self.get_memory_u8();
        self.pc += 1;

        let rst = op(self.registers[reg as usize]);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
    pub fn immediate_logic<F>(&mut self, op: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let reg = self.get_memory_u8();
        self.pc += 1;

        let val = self.add_high_low();

        let rst = op(self.registers[reg as usize], val);

        self.registers[reg as usize] = rst;
        self.update_flags(rst);
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            lstar: 0,
            cpl: 0,
            kernel_gs_base: 0,

            pc: 0,
            memory: [0; 65536],
            flags: 0,
        }
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    pub fn get_memory_u8(&self) -> u8 {
        self.memory[self.pc as usize]
    }

    pub fn get_memory_u8_as_u16(&self) -> u16 {
        self.memory[self.pc as usize] as u16
    }

    ///이 함수는 프로그램 카운터를 두칸 앞으로 옮김 `self.pc += 2`
    ///
    /// Advances the program counter by two. `self.pc += 2`
    pub fn add_high_low(&mut self) -> u16 {
        let low = self.get_memory_u8_as_u16();
        self.pc += 1;

        let high = self.get_memory_u8_as_u16();
        self.pc += 1;

        low | (high << 8)
    }
}
