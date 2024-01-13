#![allow(dead_code)]
#![allow(unused_imports)]

mod pia;
mod wmsboard;

use emu6800::{
    cpu::{decoder::print_it, diss, CpuResult, Machine, RegisterFile},
    emucore,
};

use emucore::mem;
use mem::{MemResult, MemoryIO};
use wmsboard::{WmsBoard, *};

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

fn step(machine: &mut WmsMachine, num: usize) -> CpuResult<()> {
    use emu6800::cpu::StepResult::*;

    for _ in 0..num {
        if !machine.about_to_interrupt() {
            let pc = machine.regs.pc;
            {
                let d = machine.diss(pc.into());

                if let Ok(d) = d {
                    println!("\n{d}");
                } else {
                    println!(
                        "Uknown: {pc:04x} : {:02x}",
                        machine.mem().inspect_byte(pc as usize).unwrap()
                    );
                    break;
                }
            }
        }

        let ret = machine.step().unwrap();

        match ret {
            Irq(pc) => println!("IRQ -> 0x{pc:04x}"),
            Reset(pc) => println!("RES -> 0x{pc:04x}"),
            Nmi(pc) => println!("NMI -> 0x{pc:04x}"),
            Step { .. } => {
                println!("{}", machine.regs);
            }
        }
    }
    Ok(())
}

fn main() {
    let prog = include_bytes!("../resources/sg.snd");
    let mut board = wmsboard::WmsBoard::new();
    board.upload_rom(prog).unwrap();

    let mut machine = WmsMachine::new(board, RegisterFile::default());
    machine.reset();
    step(&mut machine, 100).unwrap();
    machine.irq();
    step(&mut machine, 10000000).unwrap();
}
