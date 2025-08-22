use super::variable::Variable;

pub enum VariableStack {
    Variable(Variable),
    Reference(u64)
} 
