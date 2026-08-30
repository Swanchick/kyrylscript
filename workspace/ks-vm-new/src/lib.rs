#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod assign;
mod call_stack;
mod data_size;
mod environment;
pub mod ir;
mod native;
mod runner;
mod stats;
pub mod types;
mod utils;
mod vm;
mod vm_helper;

pub use assign::Assign;
pub use call_stack::CallStack;
pub use environment::variable::{
    BOOLEAN_TYPE, FLOAT_TYPE, FUNCTION_TYPE, INT_TYPE, NULL_TYPE, STACK_TYPE, STRING_TYPE,
};
pub use environment::{Collection, Function, GVS, Stack, Variable};
pub use native::{KsCall, NativeCall, NativeHelper, NativeRegistry};
pub use runner::Runner;
pub use vm::VM;
pub use vm_helper::VMHelper;

pub use types::VMResult;
pub use utils::VMError;
