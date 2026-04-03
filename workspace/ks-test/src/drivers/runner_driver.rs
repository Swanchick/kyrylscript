use ks_vm_new::{GVS, Runner};

pub struct RunnerDriver {
    pub gvs: GVS,
    pub runner: Runner,
}

impl RunnerDriver {
    pub fn new(runner: Runner, gvs: GVS) -> Self {
        Self { runner, gvs }
    }
}
