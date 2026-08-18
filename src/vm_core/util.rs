use std::fmt;

pub(crate) use crate::vm_core::error::{
    IVT_QUOTA, IVT_START, KSORCE_QUOTA, KSORCE_START, USER_QUOTA, USER_START, VmErr,
};
use crate::{vm::Vm, vm_core::VmCore};

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
