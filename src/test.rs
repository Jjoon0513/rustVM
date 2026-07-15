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
    use crate::vm_core::{
        VmCore,
        util::{KSORCE_QUOTA, VmErr},
    };
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
