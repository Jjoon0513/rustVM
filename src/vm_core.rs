pub mod core_settings;

use crate::vm::Vm;
use std::fs::File;

pub struct VmCore {
    vm: Vm,
}

#[allow(dead_code)]
impl VmCore {
    pub fn run(&mut self) {
        while !self.vm.halt {
            self.vm.step();
        }
    }

    /// # system OpCode not supported!!
    /// > (ex: hlt, syscall, sysret, cli, etc...)
    pub fn run_bin(&mut self, bin: Vec<u8>) {
        self.vm.memory[..bin.len()].copy_from_slice(&bin);
        self.vm.pc = 0x0000;
        self.vm.cpl = 3;

        for _ in 0..bin.len() {
            self.vm.step();
            if self.vm.halt {
                break;
            }
        }
    }

    pub fn run_max(&mut self, max_steps: u64) -> bool {
        let mut steps = 0;
        while !self.vm.halt && steps < max_steps {
            self.vm.step();
            steps += 1;
        }
        self.vm.halt // true = 정상 hlt, false = 스텝 초과
    }

    pub fn new() -> Self {
        VmCore { vm: Vm::new() }
    }

    pub fn get_vm(&mut self) -> &mut Vm {
        &mut self.vm
    }

    pub fn get_vm_ref(&self) -> &Vm {
        &self.vm
    }
}

pub fn load_bin_from_file(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    use std::io::Read;
    file.read_to_end(&mut buffer).expect("Failed to read file");
    buffer
}
