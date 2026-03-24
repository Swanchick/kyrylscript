mod ir;
mod runner;
mod stats;
mod types;
mod vgs;
mod vm;

pub use ir::constant::Constant;
pub use ir::instructions::Instruction;
pub use ir::program::Program;
pub use runner::Runner;
pub use vm::VM;
