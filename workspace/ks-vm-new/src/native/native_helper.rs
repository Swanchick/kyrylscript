use crate::{GVS, Runner};

pub struct NativeHelper<'a> {
    pub runner: &'a mut Runner,
    pub gvs: &'a mut GVS,
}

// For now we give the whole access to the languge
//
// Todo:
// Make defined functions to safely control the language
