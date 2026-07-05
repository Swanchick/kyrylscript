#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use super::{KsCall, NativeHelper};
use crate::types::Arguments;
use crate::{GVS, Runner, VMError, VMResult};

pub struct NativeRegistry {
    pub functions: Vec<Box<dyn KsCall>>,
}

impl NativeRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn call(
        &mut self,
        index: usize,
        arguments: Arguments,
        runner: &mut Runner,
        gvs: &mut GVS,
    ) -> VMResult<()> {
        let function = self.functions.get_mut(index).ok_or(VMError::from(format!(
            "Cannot find function with index {}",
            index
        )))?;

        let helper = NativeHelper { runner, gvs };

        function.call(arguments, helper)?;

        Ok(())
    }
}
