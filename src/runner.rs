use std::ops::RemAssign;

use emu6800::cpu::CpuResult;

use super::trace::Trace;
use super::WmsMachine;

#[derive(Default)]
pub enum Command {
    #[default]
    Idle,
    Reset,
    Run(usize),
    RunTo(u16),
    Poke(usize,u8),
    Irq,
    Trace(bool),
}

pub struct Runner {
    pub command: Command,
    pub trace: Trace,
    pub m: WmsMachine,
}

impl Runner {
    pub fn new() -> Self {
        panic!()
    }
    pub fn exec_command(&mut self, _c : &Command) -> CpuResult<()> {
        Ok(())
    }

    pub fn run(&mut self,commands: &[Command]) -> CpuResult<()> {

        for c in commands {
            self.exec_command(c)?
        }

        Ok(())
    }
}

