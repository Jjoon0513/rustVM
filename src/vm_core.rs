use crate::vm::Vm;

pub struct Core {
    vm: Vm,
}

impl Core {
    pub fn run(&mut self) {
        while !self.vm.halt {
            self.vm.step();
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
}