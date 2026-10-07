#[cfg(test)]
mod user_tests {
    use crate::vm_core::{VmCore, load_bin_from_file};

    #[test]
    fn test_movi_r0_10() {
        let mut vm_core = VmCore::new();
        let bin = load_bin_from_file("./asm/test/test_movi_r0_10.bin");
        vm_core.run_bin(Vec::from(bin));
        assert_eq!(vm_core.get_vm_ref().registers[0], 10);
    }

    #[test]
    fn test_add_r0_r1() {
        let mut vm_core = VmCore::new();
        let bin = load_bin_from_file("./asm/test/test_add_r0_r1.bin");
        vm_core.run_bin(Vec::from(bin));
        assert_eq!(vm_core.get_vm_ref().registers[0], 30);
    }

    #[test]
    fn test_div_10_by_3() {
        let mut vm_core = VmCore::new();
        let bin = load_bin_from_file("./asm/test/test_div_10_by_3.bin");
        vm_core.run_bin(Vec::from(bin));
        assert_eq!(vm_core.get_vm_ref().registers[13], 3); // Quotient
        assert_eq!(vm_core.get_vm_ref().registers[12], 1); // Remainder
    }

    #[test]
    fn test_push_pop() {
        let mut vm_core = VmCore::new();
        let bin = load_bin_from_file("./asm/test/test_push_pop.bin");
        vm_core.run_bin(Vec::from(bin));
        assert_eq!(vm_core.get_vm_ref().registers[3], 3);
        assert_eq!(vm_core.get_vm_ref().registers[4], 2);
        assert_eq!(vm_core.get_vm_ref().registers[5], 1);
    }
}

#[cfg(test)]
mod full_tests {
    use crate::vm_core::{VmCore, load_bin_from_file};

    #[test]
    fn hello_jjoon() {
        let mut vm_core = VmCore::new();
        let userprogram = load_bin_from_file("./asm/test/hello_jjoon.bin");
        let kernelprogram = load_bin_from_file("./asm/os.bin");
        let interrupt_vector_table = load_bin_from_file("./asm/interrupt.bin");
        vm_core.set_kernel_memory(kernelprogram);
        vm_core.set_user_memory(userprogram);
        vm_core.set_interrupt_vector_table(interrupt_vector_table);

        vm_core.get_vm().pc = 0x0100; // 유저 진입점으로 바로 점프
        vm_core.get_vm().cpl = 3; // 유저모드
        vm_core.get_vm().lstar = 0xC100; // syscall 진입점 = syc: 라벨 주소

        let result = vm_core.run_max(1000);
        assert_eq!(result, true);
    }

    #[test]
    fn hello_jjoon_loop() {
        let mut vm_core = VmCore::new();
        let userprogram = vec![
            0b00001000, 0b00000101, 0b00000000, 0b00000000, 0b00001000, 0b00000100, 0b00000101,
            0b00000000, 0b00001000, 0b00000000, 0b00000001, 0b00000000, 0b00001000, 0b00000010,
            0b00001110, 0b00000000, 0b00001000, 0b00000001, 0b00100111, 0b00000001, 0b00000001,
            0b00011100, 0b00000101, 0b00000100, 0b00110001, 0b00100010, 0b00000001, 0b00011010,
            0b00000100, 0b00000001, 0b00000000, 0b00110000, 0b00010100, 0b00000001, 0b00001000,
            0b00000000, 0b00000101, 0b00000000, 0b00000001, 0b01001000, 0b01100101, 0b01101100,
            0b01101100, 0b01101111, 0b00101100, 0b00100000, 0b01001010, 0b01101010, 0b01101111,
            0b01101111, 0b01101110, 0b00100001, 0b00001010,
        ]; //한번 해보고 싶었어요
        //  let userprogram = load_bin_from_file("./asm/test/hello_jjoon_loop.bin");
        let kernelprogram = load_bin_from_file("./asm/os.bin");
        let interrupt_vector_table = load_bin_from_file("./asm/interrupt.bin");
        vm_core.set_kernel_memory(kernelprogram);
        vm_core.set_user_memory(userprogram);
        vm_core.set_interrupt_vector_table(interrupt_vector_table);

        vm_core.get_vm().pc = 0x0100; // 유저 진입점으로 바로 점프
        vm_core.get_vm().cpl = 3; // 유저모드
        vm_core.get_vm().lstar = 0xC100; // syscall 진입점 = syc: 라벨 주소

        let result = vm_core.run_max(1000);
        assert_eq!(result, true);
    }
}

mod err_tests {
    use crate::vm_core::err::{VmErr, KSORCE_QUOTA};
    use crate::vm_core::VmCore;

    #[test]
    fn stack_head_captures_correct_overflow_bytes() {
        let mut vm_core = VmCore::new();

        let mut data = vec![0xAAu8; KSORCE_QUOTA];
        data.extend_from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55]);

        let result = vm_core.set_kernel_memory(data);
        match result {
            Err(err) => {
                println!("{}", err);

                if let VmErr::KernelMemoryOverflow { .. } = &err {
                } else {
                    panic!("wrong variant");
                }
            }
            _ => panic!("expected overflow error"),
        }
    }
}

#[cfg(test)]
mod zr_tests {
    use crate::vm::REG_ZR;
    use crate::vm_core::VmCore;

    // movi r1, 0x1234  ← 직전 명령의 PC 진행량이 맞았는지 확인하는 마커
    // (hlt는 유저 모드에서 GP 인터럽트 → pc=0 으로 되돌아가 프로그램이 재실행되므로 쓰지 않음.
    //  run_bin은 bin.len() 스텝만 돌고, 프로그램 뒤는 0x00(nop)이라 안전)
    const MARKER: [u8; 4] = [0x08, 0x01, 0x34, 0x12];

    /// prog 실행 후 ZR==0 이고 마커(R1==0x1234)까지 도달했는지 검사
    fn run_and_check(prog: &[u8]) -> VmCore {
        let mut bin = prog.to_vec();
        bin.extend_from_slice(&MARKER);
        let mut core = VmCore::new();
        core.run_bin(bin);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[REG_ZR], 0, "ZR이 오염됨");
        assert_eq!(vm.registers[1], 0x1234, "PC 진행량이 어긋남 (마커 미도달)");
        core
    }

    #[test]
    fn subi_zr() {
        run_and_check(&[0x1A, 0x10, 0x01, 0x00]);
    }

    #[test]
    fn subr_zr() {
        run_and_check(&[0x08, 0x00, 0x05, 0x00, 0x1B, 0x10, 0x00]);
    }

    #[test]
    fn addi_zr() {
        run_and_check(&[0x18, 0x10, 0x07, 0x00]);
    }

    #[test]
    fn addr_zr() {
        run_and_check(&[0x08, 0x00, 0x05, 0x00, 0x19, 0x10, 0x00]);
    }

    #[test]
    fn xor_zr() {
        run_and_check(&[0x08, 0x00, 0x05, 0x00, 0x2A, 0x10, 0x00]);
    }

    #[test]
    fn or_and_zr() {
        run_and_check(&[0x08, 0x00, 0x05, 0x00, 0x29, 0x10, 0x00, 0x28, 0x10, 0x00]);
    }

    #[test]
    fn andi_zr() {
        run_and_check(&[0x2B, 0x10, 0xFF, 0xFF]);
    }

    #[test]
    fn ori_zr() {
        run_and_check(&[0x2C, 0x10, 0xFF, 0xFF]);
    }

    #[test]
    fn xori_zr() {
        run_and_check(&[0x2D, 0x10, 0xFF, 0xFF]);
    }

    #[test]
    fn not_zr() {
        run_and_check(&[0x2E, 0x10]);
    }

    #[test]
    fn shli_zr() {
        run_and_check(&[0x08, 0x00, 0x01, 0x00, 0x60, 0x10, 0x03]);
    }

    #[test]
    fn loadi_zr() {
        run_and_check(&[0x51, 0x10, 0x00, 0x08]);
    }

    #[test]
    fn loadr_zr() {
        run_and_check(&[0x50, 0x10, 0x00]);
    }

    #[test]
    fn movi_movr_zr() {
        run_and_check(&[0x08, 0x10, 0xAA, 0xAA, 0x08, 0x00, 0x05, 0x00, 0x09, 0x10, 0x00]);
    }

    #[test]
    fn zr_reads_as_zero_after_write() {
        // subi zr,1 직후 movr r2, zr → r2 == 0 이어야 함
        let core = run_and_check(&[0x1A, 0x10, 0x01, 0x00, 0x09, 0x02, 0x10]);
        assert_eq!(core.get_vm_ref().registers[2], 0);
    }

    #[test]
    fn pop_zr_discards_value_and_balances_stack() {
        // movi r0,0x1111 ; push r0 ; pop zr  → SP 원복
        let mut bin = vec![0x08, 0x00, 0x11, 0x11, 0x40, 0x00, 0x41, 0x10];
        bin.extend_from_slice(&MARKER);
        let mut core = VmCore::new();
        let usp_before = core.get_vm_ref().usp;
        core.run_bin(bin);
        let vm = core.get_vm_ref();
        assert_eq!(vm.usp, usp_before, "pop zr 이 스택을 pop하지 않음");
        assert_eq!(vm.registers[REG_ZR], 0);
        assert_eq!(vm.registers[1], 0x1234);
    }
}

#[cfg(test)]
mod isa_v2_tests {
    //! ISA v2 확장 명령어 테스트.
    //! 아래 바이트열은 rustVM-Assembler(rva)의 encoder 테스트와 **동일한 인코딩**을 쓴다.
    //! 한쪽 레포의 opcode/피연산자 순서가 바뀌면 양쪽 테스트가 같이 깨져서 동기화 오류를 잡아낸다.
    use crate::vm::{CF, DEFAULT_USER_STACK_ADDR, OF, REG_ZR, SF, ZF};
    use crate::vm_core::VmCore;

    fn movi(r: u8, v: u16) -> Vec<u8> {
        vec![0x08, r, v as u8, (v >> 8) as u8]
    }

    /// prog를 주소 0에 올리고 유저 모드(cpl=3)로 `steps`번 실행. pokes는 실행 전에 메모리에 써 둘 값.
    fn run_steps(prog: &[u8], pokes: &[(usize, u8)], steps: usize) -> VmCore {
        let mut core = VmCore::new();
        {
            let vm = core.get_vm();
            for &(addr, v) in pokes {
                vm.memory[addr] = v;
            }
            vm.memory[..prog.len()].copy_from_slice(prog);
            vm.pc = 0;
            vm.cpl = 3;
            for _ in 0..steps {
                vm.step();
            }
        }
        core
    }

    /// prog 길이만큼 실행 (뒤쪽 0x00은 nop)
    fn run(prog: &[u8], pokes: &[(usize, u8)]) -> VmCore {
        run_steps(prog, pokes, prog.len())
    }

    fn cat(parts: &[&[u8]]) -> Vec<u8> {
        parts.iter().flat_map(|p| p.iter().copied()).collect()
    }

    // ───────────── ADC / SBB ─────────────

    #[test]
    fn adci_propagates_carry_for_32bit_add() {
        // 0x0001_FFFF + 1 = 0x0002_0000
        let prog = cat(&[
            &movi(0, 0xFFFF),
            &movi(1, 1),
            &[0x18, 0, 1, 0], // addi r0, 1  → r0=0, CF=1
            &[0x10, 1, 0, 0], // adci r1, 0  → r1 = 1 + 0 + CF
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!((vm.registers[1], vm.registers[0]), (2, 0));
    }

    #[test]
    fn adcr_propagates_carry() {
        let prog = cat(&[
            &movi(0, 0xFFFF),
            &movi(1, 1),
            &movi(2, 1),
            &movi(3, 0),
            &[0x19, 0, 2], // addr r0, r2
            &[0x11, 1, 3], // adcr r1, r3
        ]);
        let vm = run(&prog, &[]);
        assert_eq!(vm.get_vm_ref().registers[1], 2);
    }

    #[test]
    fn adc_without_carry_is_plain_add() {
        let prog = cat(&[&movi(1, 5), &[0x10, 1, 3, 0]]); // adci r1, 3
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[1], 8);
        assert!(!vm.get_flag(CF));
    }

    #[test]
    fn adc_sets_overflow_and_sign() {
        let prog = cat(&[
            &movi(4, 0xFFFF),
            &[0x18, 4, 1, 0], // addi r4, 1 → CF=1
            &movi(2, 0x7FFF),
            &[0x10, 2, 0, 0], // adci r2, 0 → 0x8000
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[2], 0x8000);
        assert!(vm.get_flag(OF) && vm.get_flag(SF));
        assert!(!vm.get_flag(CF) && !vm.get_flag(ZF));
    }

    #[test]
    fn adc_carry_out_when_carry_in_overflows() {
        // 0xFFFF + 0 + CF(1) → 0, CF=1, ZF=1
        let prog = cat(&[
            &movi(4, 0xFFFF),
            &[0x18, 4, 1, 0], // CF=1
            &movi(2, 0xFFFF),
            &[0x10, 2, 0, 0],
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[2], 0);
        assert!(vm.get_flag(CF) && vm.get_flag(ZF));
    }

    #[test]
    fn sbbi_propagates_borrow_for_32bit_sub() {
        // 0x0002_0000 - 1 = 0x0001_FFFF
        let prog = cat(&[
            &movi(0, 0),
            &movi(1, 2),
            &[0x1A, 0, 1, 0], // subi r0, 1 → r0=0xFFFF, CF=1(borrow)
            &[0x12, 1, 0, 0], // sbbi r1, 0
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!((vm.registers[1], vm.registers[0]), (1, 0xFFFF));
    }

    #[test]
    fn sbbr_propagates_borrow() {
        let prog = cat(&[
            &movi(0, 0),
            &movi(1, 2),
            &movi(2, 1),
            &movi(3, 0),
            &[0x1B, 0, 2], // subr r0, r2
            &[0x13, 1, 3], // sbbr r1, r3
        ]);
        let core = run(&prog, &[]);
        assert_eq!(core.get_vm_ref().registers[1], 1);
    }

    #[test]
    fn sbb_flags_borrow_and_sign() {
        let prog = cat(&[
            &movi(5, 0),
            &[0x1A, 5, 1, 0], // subi r5, 1 → CF=1
            &movi(1, 0),
            &[0x12, 1, 0, 0], // sbbi r1, 0 → 0 - 0 - 1
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[1], 0xFFFF);
        assert!(vm.get_flag(CF) && vm.get_flag(SF) && !vm.get_flag(ZF));
    }

    // ───────────── SAR ─────────────

    fn sari_case(initial: u16, amount: u8) -> (u16, bool) {
        let prog = cat(&[&movi(0, initial), &[0x64, 0, amount]]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        (vm.registers[0], vm.get_flag(CF))
    }

    #[test]
    fn sari_keeps_sign() {
        assert_eq!(sari_case(0x8000, 1), (0xC000, false));
        assert_eq!(sari_case(0x0003, 1), (0x0001, true));
        assert_eq!(sari_case(0x4000, 3), (0x0800, false));
        assert_eq!(sari_case(0xFFF0, 4), (0xFFFF, false));
    }

    #[test]
    fn sari_large_amounts_fill_with_sign() {
        assert_eq!(sari_case(0x8000, 16), (0xFFFF, true));
        assert_eq!(sari_case(0x8000, 20), (0xFFFF, true));
        assert_eq!(sari_case(0x4000, 20), (0x0000, false));
        assert_eq!(sari_case(0x4000, 255), (0x0000, false)); // 호스트 패닉 없음
    }

    #[test]
    fn sari_zero_amount_changes_nothing_but_flags() {
        assert_eq!(sari_case(0x1234, 0), (0x1234, false));
    }

    #[test]
    fn sarr_uses_register_amount() {
        let prog = cat(&[&movi(4, 0xFF00), &movi(5, 4), &[0x65, 4, 5]]);
        let core = run(&prog, &[]);
        assert_eq!(core.get_vm_ref().registers[4], 0xFFF0);
    }

    #[test]
    fn sarr_huge_amount_does_not_panic() {
        let prog = cat(&[&movi(4, 0x8000), &movi(5, 0xFFFF), &[0x65, 4, 5]]);
        let core = run(&prog, &[]);
        assert_eq!(core.get_vm_ref().registers[4], 0xFFFF);
    }

    // ───────────── 간접 점프 / 호출 ─────────────

    #[test]
    fn jmpr_jumps_to_register_value() {
        let prog = cat(&[
            &movi(0, 10),
            &[0x3B, 0],       // jmpr r0
            &movi(1, 0x0BAD), // 건너뛰어야 함
            &movi(2, 0x1234), // 주소 10
        ]);
        let core = run(&prog, &[]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[1], 0);
        assert_eq!(vm.registers[2], 0x1234);
    }

    #[test]
    fn callr_calls_and_returns_with_balanced_stack() {
        let prog = cat(&[
            // r7을 쓰는 이유: 피연산자 바이트가 0x07(iret → 유저 모드에서 #GP)이라서,
            // 복귀 주소가 한 칸이라도 어긋나면 그 바이트를 실행하게 되어 테스트가 실패한다. (r0이면 nop이라 못 잡음)
            &movi(7, 14),
            &[0x4A, 7],       // callr r7  (복귀 주소 6)
            &movi(2, 0x2222), // 6..9  : 복귀 후 실행
            &[0x30, 20, 0],   // 10..12: jmp 20
            &[0x00],          // 13
            &movi(1, 0x1111), // 14..17: 함수 본문
            &[0x49],          // 18    : ret
            &[0x00],          // 19
        ]);
        // 정확히 6 스텝(movi, callr, movi, ret, movi, jmp)만 실행한다.
        // 이 VM은 예외 후 "예외를 낸 명령의 다음"으로 복귀하므로, 복귀 주소가 어긋나 #GP 우회가 생겨도
        // 최종 상태는 같아진다. 스텝 수를 고정하면 우회가 생긴 순간 jmp에 도달하지 못해 실패한다.
        let core = run_steps(&prog, &[], 6);
        let vm = core.get_vm_ref();
        assert_eq!(vm.pc, 20, "정확히 6 스텝 안에 jmp 20까지 도달해야 함");
        assert_eq!(vm.registers[1], 0x1111);
        assert_eq!(vm.registers[2], 0x2222);
        assert_eq!(vm.cpl, 3);
        assert_eq!(vm.usp, DEFAULT_USER_STACK_ADDR, "callr/ret 후 스택이 원복되어야 함");
    }

    // ───────────── 바이트 / 오프셋 load·store ─────────────

    #[test]
    fn loadb_zero_extends() {
        let prog = cat(&[&movi(1, 0xFFFF), &movi(0, 0x2000), &[0x54, 1, 0]]);
        let core = run(&prog, &[(0x2000, 0xAB), (0x2001, 0xCD)]);
        assert_eq!(core.get_vm_ref().registers[1], 0x00AB);
    }

    #[test]
    fn storeb_writes_only_one_byte() {
        let prog = cat(&[&movi(2, 0x1234), &movi(3, 0x2001), &[0x58, 3, 2]]);
        let core = run(&prog, &[(0x2000, 0xAB), (0x2001, 0xCD), (0x2002, 0x77)]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.memory[0x2000], 0xAB);
        assert_eq!(vm.memory[0x2001], 0x34);
        assert_eq!(vm.memory[0x2002], 0x77, "이웃 바이트를 건드리면 안 됨");
    }

    #[test]
    fn offset_load_store_with_negative_offsets() {
        let prog = cat(&[
            &movi(0, 0x1FFE),
            &[0x55, 1, 0, 2, 0],       // loado  r1, r0, 2     → mem16[0x2000]
            &movi(0, 0x2004),
            &[0x55, 2, 0, 0xFC, 0xFF], // loado  r2, r0, -4    → mem16[0x2000]
            &[0x56, 3, 0, 0xFD, 0xFF], // loadbo r3, r0, -3    → mem8[0x2001]
            &movi(4, 0xBEEF),
            &[0x59, 0, 0xFC, 0xFF, 4], // storeo  r0, -4, r4   → mem16[0x2000] = 0xBEEF
            &movi(5, 0x0142),
            &[0x5A, 0, 0xFD, 0xFF, 5], // storebo r0, -3, r5   → mem8[0x2001] = 0x42
        ]);
        let core = run(&prog, &[(0x2000, 0xAB), (0x2001, 0xCD)]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[1], 0xCDAB);
        assert_eq!(vm.registers[2], 0xCDAB);
        assert_eq!(vm.registers[3], 0x00CD);
        assert_eq!(vm.memory[0x2000], 0xEF);
        assert_eq!(vm.memory[0x2001], 0x42);
        assert_eq!(vm.memory[0x2002], 0x00);
    }

    #[test]
    fn effective_address_wraps_instead_of_panicking() {
        let prog = cat(&[
            &movi(0, 0xFFFF),
            &[0x55, 1, 0, 0, 0], // loado r1, r0, 0 → mem[0xFFFF] | mem[0x0000]<<8 (둘째 바이트 래핑)
            &movi(2, 0xABCD),
            &[0x59, 0, 0, 0, 2], // storeo r0, 0, r2 → mem[0xFFFF]=CD, mem[0x0000]=AB
            &[0x56, 3, 0, 1, 0], // loadbo r3, r0, 1 → ea = 0x0000
        ]);
        let core = run(&prog, &[(0xFFFF, 0x11)]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[1], 0x0811); // mem[0]은 프로그램 첫 바이트(0x08)
        assert_eq!(vm.memory[0xFFFF], 0xCD);
        assert_eq!(vm.registers[3], 0x00AB);
    }

    #[test]
    fn uart_store_from_user_mode_raises_gp() {
        let prog = cat(&[&movi(0, 0xC000), &movi(1, 0x41), &[0x58, 0, 1]]); // storeb r0, r1
        let core = run_steps(&prog, &[], 3);
        let vm = core.get_vm_ref();
        assert_eq!(vm.cpl, 0, "#GP 인터럽트로 커널 모드 진입");
        assert_eq!(vm.memory[0xC000], 0);
    }

    #[test]
    fn uart_store_via_offset_from_user_mode_raises_gp() {
        let prog = cat(&[&movi(0, 0xBFFF), &movi(1, 0x41), &[0x5A, 0, 1, 0, 1]]); // storebo r0, 1, r1 → 0xC000
        let core = run_steps(&prog, &[], 3);
        assert_eq!(core.get_vm_ref().cpl, 0);
    }

    // ───────────── ZR / 기존 동작 ─────────────

    #[test]
    fn zr_destination_stays_zero_and_pc_is_correct() {
        let prog = cat(&[
            &movi(0, 0x2000),
            &[0x54, 0x10, 0],          // loadb  zr, r0
            &[0x55, 0x10, 0, 0, 0],    // loado  zr, r0, 0
            &[0x56, 0x10, 0, 0, 0],    // loadbo zr, r0, 0
            &[0x10, 0x10, 1, 0],       // adci   zr, 1
            &[0x12, 0x10, 1, 0],       // sbbi   zr, 1
            &[0x64, 0x10, 1],          // sari   zr, 1
            &movi(1, 0x1234),          // 마커: 앞 명령들의 PC 진행량이 맞아야 도달
        ]);
        let core = run(&prog, &[(0x2000, 0xFF), (0x2001, 0xFF)]);
        let vm = core.get_vm_ref();
        assert_eq!(vm.registers[REG_ZR], 0);
        assert_eq!(vm.registers[1], 0x1234);
    }

    #[test]
    fn undefined_neighbor_opcodes_are_still_invalid() {
        // 새로 배정하지 않은 슬롯은 여전히 #UD (예: 0x14, 0x57, 0x66)
        for op in [0x14u8, 0x57, 0x66] {
            let core = run_steps(&[op], &[], 1);
            assert_eq!(core.get_vm_ref().cpl, 0, "opcode {op:#04x} 은 #UD 여야 함");
        }
    }
}
