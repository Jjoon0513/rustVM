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
