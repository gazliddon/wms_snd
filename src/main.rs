#![allow(dead_code)]
#![allow(unused_imports)]

mod pia;
mod wmsboard;

use emu6800::{
    cpu::{diss, Machine, RegisterFile},
    emucore,
};

use emucore::mem;

use wmsboard::{WmsBoard, *};

use mem::{MemResult, MemoryIO};

pub type WmsMachine = Machine<WmsBoard, RegisterFile>;

use emu6800::cpu_core::{Isa, IsaDatabase};

lazy_static::lazy_static! {
    static ref DBASE : IsaDatabase = {
        let txt = include_str!("../../crates/emu6800/resources/opcodes6800.json");
        let isa: Isa = serde_json::from_str(txt).unwrap();
        IsaDatabase::new(&isa)
    };
}

pub struct Runner {
    machine: WmsMachine,
    cycle: usize,
}

fn main() {
    let prog = include_bytes!("../resources/sg.snd");
    let mut board = wmsboard::WmsBoard::new();
    board.upload_rom(prog).unwrap();

    let mut machine = WmsMachine::new(board, RegisterFile::default());

    machine.reset().unwrap();

    for _i in 0..100 {
        let pc = machine.regs.pc as usize;

        let d = diss(machine.mem(), pc);

        if let Ok(d) = d {
            println!( "{d}");
        } else {
            println!(
                "Uknown: {pc:04x} : {:02x}",
                machine.mem().inspect_byte(pc as usize).unwrap()
            );
            break;
        }

        machine.step().unwrap();
    }
}
