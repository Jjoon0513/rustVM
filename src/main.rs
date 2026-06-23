use crate::vm::Vm;

mod exec;
mod step;
mod util;
mod vm;

fn main() {
    let mut vm = Vm::new();
    loop {
        vm.step();
    }
}
