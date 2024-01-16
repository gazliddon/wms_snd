#![allow(dead_code)]
#![allow(unused_imports)]

mod pia;
mod wmsboard;

use emu6800::{
    cpu::{decoder::print_it, diss, CpuResult, Machine, RegisterFile, StepResult},
    emucore,
};

use emucore::mem;
use mem::{MemResult, MemoryIO};
use wmsboard::{WmsBoard, *};

pub type WmsMachine = Machine<WmsBoard, RegisterFile>;

use emu6800::cpu_core::{Isa, IsaDatabase};

pub struct Runner {
    machine: WmsMachine,
    cycle: usize,
}

fn play_sample(machine: &mut WmsMachine, num: usize, sound: u8) -> CpuResult<Vec<u8>> { 
    use emu6800::cpu::{CpuState, StepResult::*};

    machine.cycle = 0;
    machine.reset();

    // execute a 1000 instructions to init the board
    for _ in 0..100 {
        machine.step().expect("preliminary step");
    }

    println!("{}", machine.regs);

    machine.mem_mut().set_sfx(!sound);
    machine.irq = true;

    // Now capture num samples from the HW
    let mut captured_sound : Vec<u8> = Vec::with_capacity(num);

    while captured_sound.len() < num {
        let ret = step(machine, 1).unwrap();

        if let Step{cycles, ..} = ret {
            let sound = machine.mem().get_dac();
            for _ in 0..cycles {
                captured_sound.push(sound)
            }
        }
    }

    println!("{}", machine.regs);

    Ok(captured_sound)
}

fn step(machine: &mut WmsMachine, num: usize) -> CpuResult<StepResult> {
    use emu6800::cpu::{CpuState, StepResult::*};

    let mut ret : StepResult = Default::default();

    for _ in 0..num {
        let this_step = machine.step().unwrap();
        ret = this_step;

        match ret {
            Irq(pc) => println!("IRQ -> 0x{pc:04x}"),
            Reset(pc) => println!("RES -> 0x{pc:04x}"),
            Nmi(pc) => println!("NMI -> 0x{pc:04x}"),
            Step { pc, .. } => {
                println!("{}", machine.regs);
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
    }

    Ok(ret)
}

// ffmpeg convert to a wav file
//ffmpeg -f s16le -ar 44.1k -ac 2 -i file.pcm file.wav
//ffmpeg -f u8 -ar 44.1k -ac 1 -i sg.pcm file.wav

fn main() {
    use std::fs::File;
    use std::io::Write;
    let prog = include_bytes!("../resources/sg.snd");
    let mut board = wmsboard::WmsBoard::new();
    board.upload_rom(prog).unwrap();

    let mut machine = WmsMachine::new(board, RegisterFile::default());
    let _ret = play_sample(&mut machine, 1024*1024, 1).expect("Playing sample");
    let mut f = File::create("sg.pcm").unwrap();

    f.write_all(&_ret[0..]).expect("Writing file");
}
