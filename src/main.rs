use crate::vm::Vm;

mod exec;
mod step;
mod vm;
mod util;

fn main() {
    let mut vm = Vm::new();
    loop {
        vm.step();
    }
}
