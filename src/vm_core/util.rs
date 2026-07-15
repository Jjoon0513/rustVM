use std::fmt;

use crate::{vm::Vm, vm_core::VmCore};

// ── 메모리 레이아웃 상수 ──────────────────────────────────────────────
pub const IVT_START: usize = 0x0000;
pub const IVT_END: usize = 0x0100; // exclusive, User 시작과 동일
pub const IVT_QUOTA: usize = IVT_END - IVT_START;

pub const USER_START: usize = 0x0100;
pub const USER_END: usize = 0xA000; // exclusive, UserStack 시작과 동일
pub const USER_QUOTA: usize = USER_END - USER_START;

pub const KSORCE_START: usize = 0xC100;
pub const KSORCE_END: usize = 0xF000; // exclusive, KStack 시작과 동일
pub const KSTACK_START: usize = 0xF000; // 이전에 0xF001로 잘못 씀 — 표 기준 수정
pub const KSORCE_QUOTA: usize = KSORCE_END - KSORCE_START;

pub const STACK_PREVIEW_LEN: usize = 8;

const _: () = assert!(KSTACK_START + STACK_PREVIEW_LEN <= 65536);

// ── 색상 (ANSI) ───────────────────────────────────────────────────────
const CYAN: &str = "\x1b[36m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

fn colored(s: &str, color: &str) -> String {
    format!("{color}{s}{RESET}")
}

pub enum VmErr {
    KernelMemoryOverflow {
        length: usize,
        quota: usize,
        data: Vec<u8>,
    },
    UserMemoryOverflow {
        length: usize,
        quota: usize,
        data: Vec<u8>,
    },
    IVTMemoryOverflow {
        length: usize,
        quota: usize,
        data: Vec<u8>,
    },
    MemoryOverflow {
        length: u16,
    },
}

impl VmErr {
    fn kernel_memory_overflow(data: Vec<u8>) -> Self {
        VmErr::KernelMemoryOverflow {
            length: data.len(),
            quota: KSORCE_QUOTA,
            data,
        }
    }

    fn user_memory_overflow(data: Vec<u8>) -> Self {
        VmErr::UserMemoryOverflow {
            length: data.len(),
            quota: USER_QUOTA,
            data,
        }
    }

    fn ivt_memory_overflow(data: Vec<u8>) -> Self {
        VmErr::IVTMemoryOverflow {
            length: data.len(),
            quota: IVT_QUOTA,
            data,
        }
    }
}

fn hex_join(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

/// KSorce/User/IVT 오버플로우 전부 공통으로 쓰는 hex-dump 박스 렌더러.
/// region_name: 문장에 쓰이는 이름 ("Kernel", "User", "IVT")
/// label / next_label: 주소 옆에 붙는 라벨 ("KSorce"/"KStack", "User"/"UserStack", "IVT"/"User")
/// region_start / region_end: 이 리전의 [start, end) 범위. end == 다음 리전의 시작 주소.
fn render_overflow_box(
    f: &mut fmt::Formatter<'_>,
    region_name: &str,
    label: &str,
    next_label: &str,
    region_start: usize,
    region_end: usize,
    length: usize,
    quota: usize,
    data: &[u8],
) -> fmt::Result {
    writeln!(
        f,
        "Error: Data larger than the {} memory quota has been entered.",
        region_name
    )?;
    writeln!(f, "Memory would be...")?;
    writeln!(f)?;

    // 헤더 hex 영역 폭 (00 01 ... 0F, 항상 47글자 고정)
    let addr_col: Vec<String> = (0..16u8).map(|i| format!("{:02X}", i)).collect();
    let hex_area_str = addr_col.join(" ");
    let hex_area_width = hex_area_str.len();
    let box_inner_width = hex_area_width + 4;

    // label / next_label 길이가 다를 수 있으니 (User=4, UserStack=9 등) 폭을 맞춤
    let label_width = label.len().max(next_label.len());
    let make_prefix =
        |addr: usize, lbl: &str| format!("0x{:04X} ({:<lw$}) ", addr, lbl, lw = label_width);
    // 순수 spacing 계산용 (border 정렬 기준)
    let prefix_len = make_prefix(0, label).len();
    let indent = " ".repeat(prefix_len);

    let bar = colored("|", CYAN);
    let border_line = format!(
        "{}{}{}",
        indent,
        bar,
        colored(&"-".repeat(box_inner_width), CYAN)
    ) + &bar;
    let frame_line = format!(
        "{}{}",
        indent,
        colored(&format!("+{}+", "-".repeat(box_inner_width)), CYAN)
    );

    writeln!(f, "{}", frame_line)?;
    writeln!(
        f,
        "{}{}  {}  {}",
        indent,
        bar,
        colored(&hex_area_str, YELLOW),
        bar
    )?;
    writeln!(f, "{}", border_line)?;

    // 1) 첫 16바이트 (region_start부터) — 유효 데이터, 초록
    let head_len = 16.min(data.len());
    let head_hex = hex_join(&data[..head_len]);
    let head_padded = format!("{:<width$}", head_hex, width = hex_area_width);
    writeln!(
        f,
        "{}{}  {}  {}",
        colored(&make_prefix(region_start, label), YELLOW),
        bar,
        colored(&head_padded, GREEN),
        bar
    )?;

    // 2) 중간 생략 구간
    let region_size = region_end - region_start;
    if region_size > 48 {
        let gap_start = region_start + 16;
        let gap_end = region_end - 32 - 1;
        let gap_label = format!("0x{:04X} ~ 0x{:04X} ", gap_start, gap_end);
        let gap_content = format!("~~{}~~", " ".repeat(hex_area_width.saturating_sub(4)));
        writeln!(
            f,
            "{}{}  {}  {}",
            colored(
                &format!("{:<width$}", gap_label, width = prefix_len),
                YELLOW
            ),
            bar,
            colored(&gap_content, YELLOW),
            bar
        )?;
    }

    // 3) quota 끝 32바이트 (region_end 직전) — 유효 데이터, 초록
    let ksorce_end = quota.min(data.len());
    let tail_start = ksorce_end.saturating_sub(32);
    let tail = &data[tail_start..ksorce_end];
    let row1_hex = hex_join(&tail[..tail.len().min(16)]);
    let row2_hex = if tail.len() > 16 {
        hex_join(&tail[16..])
    } else {
        String::new()
    };
    let row1_addr = region_end.saturating_sub(32);
    let row2_addr = region_end.saturating_sub(16);
    let row1_padded = format!("{:<width$}", row1_hex, width = hex_area_width);
    let row2_padded = format!("{:<width$}", row2_hex, width = hex_area_width);
    writeln!(
        f,
        "{}{}  {}  {}",
        colored(&make_prefix(row1_addr, label), YELLOW),
        bar,
        colored(&row1_padded, GREEN),
        bar
    )?;
    writeln!(
        f,
        "{}{}  {}  {}",
        colored(&make_prefix(row2_addr, label), YELLOW),
        bar,
        colored(&row2_padded, GREEN),
        bar
    )?;

    writeln!(f, "{}", border_line)?;

    // 4) 다음 리전 침범 미리보기 — overflow, 빨강
    let overflow_start = quota;
    let overflow_end = (overflow_start + STACK_PREVIEW_LEN).min(data.len());
    let overflow_bytes: &[u8] = if overflow_start < data.len() {
        &data[overflow_start..overflow_end]
    } else {
        &[]
    };
    let total_overflow = length - quota;
    let suffix = if total_overflow > STACK_PREVIEW_LEN {
        " ..."
    } else {
        ""
    };
    let overflow_content = format!("{}{}", hex_join(overflow_bytes), suffix);
    let overflow_padded = format!("{:<width$}", overflow_content, width = hex_area_width);
    writeln!(
        f,
        "{}{}  {}  {}",
        colored(&make_prefix(region_end, next_label), YELLOW),
        bar,
        colored(&overflow_padded, RED),
        bar
    )?;

    // 5) OF 마커 — 빨강
    let of_markers = "OF ".repeat(overflow_bytes.len()).trim_end().to_string();
    let of_content = format!("{}{}", of_markers, suffix);
    let of_padded = format!("{:<width$}", of_content, width = hex_area_width);
    writeln!(
        f,
        "{}{}  {}  {}",
        indent,
        bar,
        colored(&of_padded, RED),
        bar
    )?;

    writeln!(f, "{}", frame_line)?;
    writeln!(f)?;

    write!(
        f,
        "= You can compress or optimize the file, or disable it with .ignore_memory_check()."
    )
}

impl fmt::Display for VmErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmErr::KernelMemoryOverflow {
                length,
                quota,
                data,
            } => render_overflow_box(
                f,
                "Kernel",
                "KSorce",
                "KStack",
                KSORCE_START,
                KSORCE_END,
                *length,
                *quota,
                data,
            ),
            VmErr::UserMemoryOverflow {
                length,
                quota,
                data,
            } => render_overflow_box(
                f,
                "User",
                "User",
                "UserStack",
                USER_START,
                USER_END,
                *length,
                *quota,
                data,
            ),
            VmErr::IVTMemoryOverflow {
                length,
                quota,
                data,
            } => render_overflow_box(
                f, "IVT", "IVT", "User", IVT_START, IVT_END, *length, *quota, data,
            ),
            VmErr::MemoryOverflow { length } => {
                write!(f, "Error: Memory overflow ({} bytes given).", length)
            }
        }
    }
}

impl VmCore {
    pub fn set_kernel_memory(&mut self, data: Vec<u8>) -> Result<&mut Self, VmErr> {
        if data.len() > KSORCE_QUOTA {
            return Err(VmErr::kernel_memory_overflow(data));
        }
        self.vm.memory[KSORCE_START..KSORCE_START + data.len()].copy_from_slice(&data);
        Ok(self)
    }

    pub fn set_user_memory(&mut self, data: Vec<u8>) -> Result<&mut Self, VmErr> {
        if data.len() > USER_QUOTA {
            return Err(VmErr::user_memory_overflow(data));
        }
        self.vm.memory[USER_START..USER_START + data.len()].copy_from_slice(&data);
        Ok(self)
    }

    pub fn set_interrupt_vector_table(&mut self, data: Vec<u8>) -> Result<&mut VmCore, VmErr> {
        if data.len() > IVT_QUOTA {
            return Err(VmErr::ivt_memory_overflow(data));
        }
        self.vm.memory[IVT_START..IVT_START + data.len()].copy_from_slice(&data);
        Ok(self)
    }

    pub fn get_vm(&mut self) -> &mut Vm {
        &mut self.vm
    }

    pub fn get_vm_ref(&self) -> &Vm {
        &self.vm
    }
}
