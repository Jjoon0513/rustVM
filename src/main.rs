#![no_std]

use crate::vm::Vm;

mod exec;
mod step;
mod vm;

fn main() {
    let mut vm = Vm::new();
    loop {
        vm.step();
    }
}
