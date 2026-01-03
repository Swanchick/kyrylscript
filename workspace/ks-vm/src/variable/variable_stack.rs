use crate::environment::Reference;

use super::variable::Variable;

#[derive(Debug)]
pub enum VariableStack {
    Variable(Variable),
    Reference(Reference),
}
