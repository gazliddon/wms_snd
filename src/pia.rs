pub const PIA_BASE: u16 = 0x400;
pub const PIA_SIZE: u16 = 0x4;
pub const PIA_LAST: u16 = PIA_BASE.wrapping_add(PIA_SIZE).wrapping_sub(1);

pub struct Pia {
    last_written: [u8; 4],
}

impl Default for Pia {
    fn default() -> Self {
        Self {
            last_written: Default::default(),
        }
    }
}

impl Pia {
    pub fn new() -> Self {
        Self {
            last_written: Default::default(),
        }
    }

    pub fn write(&mut self, a: u8, v: u8) {
        self.last_written[a as usize] = v;
    }
    pub fn read(&mut self, _a: u8) -> u8 {
        0
    }
}

