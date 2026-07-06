#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod assign;
mod call_stack;
mod environment;
pub mod ir;
mod native;
mod runner;
mod stats;
pub mod types;
mod utils;
mod vm;

pub use assign::Assign;
pub use call_stack::CallStack;
pub use environment::variable::{
    BOOLEAN_TYPE, FLOAT_TYPE, FUNCTION_TYPE, INT_TYPE, NULL_TYPE, STACK_TYPE, STRING_TYPE,
};
pub use environment::{Collection, Function, GVS, Stack, Variable};
pub use ir::constant::Constant;
pub use ir::instructions::Instruction;
pub use ir::program::Program;
pub use native::{KsCall, NativeCall, NativeHelper, NativeRegistry};
pub use runner::Runner;
pub use vm::VM;

pub use types::VMResult;
pub use utils::VMError;
