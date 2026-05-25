mod environment;
mod ir;
mod runner;
mod stats;
mod types;
mod utils;
mod vm;

pub use environment::{Collection, GVS, Stack, Variable};
pub use ir::constant::Constant;
pub use ir::instructions::Instruction;
pub use ir::program::Program;
pub use runner::Runner;
pub use vm::VM;
