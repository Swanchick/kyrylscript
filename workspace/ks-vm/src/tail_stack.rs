use crate::variable::var_info::VarInfo;

#[derive(Debug)]
pub enum TailStack {
    Variable(String),
    Index { index: usize, info: VarInfo },
    Module { name: String, info: VarInfo },
}
