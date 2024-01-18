use super::trace::Trace;
use super::wmsboard;
use super::WmsMachine;
use emu6800::cpu::{CpuResult, RegisterFile};

#[derive(Default)]
pub enum Command {
    #[default]
    Idle,
    Reset,
    Run(usize),
    RunTo(u16),
    Poke(usize, u8),
    Irq,
    Trace(bool),
}

pub struct Runner {
    pub trace: Trace,
    pub m: WmsMachine,
}

pub fn make_machine() -> WmsMachine {
    let prog = include_bytes!("../resources/sg.snd");
    let mut board = wmsboard::WmsBoard::new();
    board.upload_rom(prog).unwrap();
    WmsMachine::new(board, RegisterFile::default())
}

impl Runner {
    pub fn new(m: WmsMachine) -> Self {
        Self {
            trace: Default::default(),
            m,
        }
    }
    pub fn exec_command(&mut self, _c: &Command) -> CpuResult<()> {
        Ok(())
    }

    pub fn run(&mut self, commands: &[Command]) -> CpuResult<()> {
        for c in commands {
            self.exec_command(c)?
        }

        Ok(())
    }
}
