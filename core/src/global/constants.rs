use crate::compiler::instruction::Instruction;

pub type Instructions = Vec<Instruction>;

pub const FUNCTION_ENCAPSULATION: &str = "__function_";
pub const ANONYNOUS_FUNCTION_ENCAPSULATION: &str = "__anonymous_";
pub const MAIN_FUNCTION: &str = "main";
pub const ITERATOR_NAME: &str = "__iter_";
pub const ITERATOR_LIST_NAME: &str = "__iter_list_";
