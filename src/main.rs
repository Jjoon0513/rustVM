use crate::vm::Vm;

mod exec;
mod step;
mod util;
mod vm;
mod vm_core;

fn main() {
    let mut vm = Vm::new();
    loop {
        vm.step();
    }
}
