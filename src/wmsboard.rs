use super::pia::*;

use emu6800::emucore::mem::{self, MemErrorTypes, MemResult, MemoryIO};

const ROM_BASE: u16 = 0xf800;
const ROM_SIZE: u16 = 0x800;
const ROM_LAST: u16 = ROM_BASE.wrapping_add(ROM_SIZE).wrapping_sub(1);

const RAM_BASE: u16 = 0x0;
const RAM_SIZE: u16 = 0x80;
const RAM_LAST: u16 = RAM_BASE.wrapping_add(RAM_SIZE).wrapping_sub(1);

pub struct WmsBoard {
    ram: [u8; RAM_SIZE as usize],
    rom: [u8; ROM_SIZE as usize],
    pia: Pia,
}

impl Default for WmsBoard {
    fn default() -> Self {
        Self {
            ram: [0; RAM_SIZE as usize],
            rom: [0; ROM_SIZE as usize],
            pia: Default::default(),
        }
    }
}

impl WmsBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn upload_rom(&mut self, src: &[u8]) -> MemResult<()> {
        if src.len() > ROM_SIZE.into() {
            panic!();
        }

        for (i,b) in src.into_iter().enumerate() {
            self.rom[i] = *b;
        }

        Ok(())
    }

    pub fn read_u8(&mut self, addr: u16) -> MemResult<u8> {
        match addr {
            RAM_BASE..=RAM_LAST => self.read_ram_u8(addr.wrapping_sub(RAM_BASE)),
            PIA_BASE..=PIA_LAST => self.read_pia_u8(addr.wrapping_sub(PIA_BASE)),
            ROM_BASE..=ROM_LAST => self.read_rom_u8(addr.wrapping_sub(ROM_BASE)),
            _ => return Err(MemErrorTypes::IllegalAddress(addr as usize)),
        }
    }

    pub fn write_u8(&mut self, addr: u16, val: u8) -> MemResult<()> {
        match addr {
            RAM_BASE..=RAM_LAST => self.write_ram_u8(addr.wrapping_sub(RAM_BASE), val),
            PIA_BASE..=PIA_LAST => self.write_pia_u8(addr.wrapping_sub(PIA_BASE), val),
            ROM_BASE..=ROM_LAST => self.write_rom_u8(addr.wrapping_sub(ROM_BASE), val),
            _ => return Err(MemErrorTypes::IllegalAddress(addr as usize)),
        }
    }

    pub fn inspect_u8(&self, addr: u16) -> MemResult<u8> {
        match addr {
            RAM_BASE..=RAM_LAST => self.inspect_ram_u8(addr.wrapping_sub(RAM_BASE)),
            PIA_BASE..=PIA_LAST => self.inspect_pia_u8(addr.wrapping_sub(PIA_BASE)),
            ROM_BASE..=ROM_LAST => self.inspect_rom_u8(addr.wrapping_sub(ROM_BASE)),
            _ => return Err(MemErrorTypes::IllegalAddress(addr as usize)),
        }
    }

    pub fn inspect_pia_u8(&self, _addr: u16) -> MemResult<u8> {
        Ok(0)
    }

    pub fn inspect_ram_u8(&self, addr: u16) -> MemResult<u8> {
        Ok(self.ram[addr as usize])
    }

    pub fn inspect_rom_u8(&self, addr: u16) -> MemResult<u8> {
        Ok(self.rom[addr as usize])
    }

    pub fn read_pia_u8(&mut self, _addr: u16) -> MemResult<u8> {
        Ok(0)
    }

    pub fn read_ram_u8(&mut self, addr: u16) -> MemResult<u8> {
        Ok(self.ram[addr as usize])
    }

    pub fn read_rom_u8(&mut self, addr: u16) -> MemResult<u8> {
        Ok(self.rom[addr as usize])
    }

    pub fn write_pia_u8(&mut self, _addr: u16, _val: u8) -> MemResult<()> {
        Ok(())
    }

    pub fn write_ram_u8(&mut self, addr: u16, val: u8) -> MemResult<()> {
        self.ram[addr as usize] = val;
        Ok(())
    }

    pub fn write_rom_u8(&mut self, _addr: u16, _val: u8) -> MemResult<()> {
        Ok(())
    }

    #[inline]
    pub fn get_addr(addr: usize) -> MemResult<u16> {
        if addr <= 0xffff {
            Ok((addr & 0xffff) as u16)
        } else {
            Err(MemErrorTypes::IllegalAddress(addr))
        }
    }
}

impl MemoryIO for WmsBoard {
    fn inspect_word(&self, addr: usize) -> mem::MemResult<u16> {
        let h = self.inspect_byte(addr)? as u16;
        let l = self.inspect_byte(addr.wrapping_add(1))? as u16;
        Ok(h.wrapping_shl(8) | l)
    }

    fn inspect_byte(&self, addr: usize) -> mem::MemResult<u8> {
        Self::get_addr(addr).and_then(|addr| self.inspect_u8(addr))
    }

    fn upload(&mut self, addr: usize, data: &[u8]) -> mem::MemResult<()> {
        for (i, b) in data.iter().enumerate() {
            self.store_byte(addr + i, *b)?
        }
        Ok(())
    }

    fn get_range(&self) -> std::ops::Range<usize> {
        0..0x10000
    }

    fn update_sha1(&self, _digest: &mut emu6800::emucore::sha1::Sha1) {
        todo!()
    }

    fn load_byte(&mut self, addr: usize) -> mem::MemResult<u8> {
        Self::get_addr(addr).and_then(|addr| self.read_u8(addr))
    }

    fn store_byte(&mut self, addr: usize, val: u8) -> mem::MemResult<()> {
        Self::get_addr(addr).and_then(|addr| self.write_u8(addr, val))
    }

    fn store_word(&mut self, addr: usize, val: u16) -> mem::MemResult<()> {
        self.store_byte(addr, val.wrapping_shr(8) as u8)?;
        self.store_byte(addr + 1, (val & 0xff) as u8)?;
        Ok(())
    }

    fn load_word(&mut self, addr: usize) -> mem::MemResult<u16> {
        let h = self.load_byte(addr)? as u16;
        let l = self.load_byte(addr + 1)? as u16;
        Ok(h.wrapping_shl(8) | l)
    }
}
