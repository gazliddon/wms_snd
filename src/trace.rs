use emu6800::{
    cpu::{decoder::print_it, diss, CpuResult, Machine, RegisterFile, StepResult},
    emucore,
    emucore::mem::MemoryIO
};
use super::WmsMachine;

#[derive(Clone, Debug)]
pub struct WmsState {
    cycle: usize,
    regs: RegisterFile,
    sha1: String,
}

impl WmsState {
    pub fn new(m: &WmsMachine) -> Self {
        let sha1 = m.mem.get_sha1_string();

        WmsState {
            cycle: m.cycle,
            sha1,
            regs: m.regs.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Trace {
    trace: Vec<WmsState>
}

impl Trace {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, m: &WmsMachine) {
        self.trace.push(WmsState::new(m))
    }
}
