use crate::vm::var_info::VarInfo;

 pub enum TailStack {
    Variable(String),
    Index {
        index: usize,
        info: VarInfo,
    },
    Module {
        name: String,
        info: VarInfo,
    },
}