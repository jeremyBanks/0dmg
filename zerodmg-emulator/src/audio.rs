use super::GameBoy;

/// Game Boy audio controller state
pub struct AudioData {
    t: u64,
    registers: [u8; 0x2F],
}

impl AudioData {
    pub fn new() -> Self {
        Self {
            t: 0,
            registers: [0; 0x2F],
        }
    }
}

pub trait AudioController {
    fn audio_cycle(&mut self);
    fn audio_register(&self, index: usize) -> u8;
    fn set_audio_register(&mut self, index: usize, value: u8);
}

impl AudioController for GameBoy {
    fn audio_cycle(&mut self) {
        self.aud.t += 1;
    }

    fn audio_register(&self, index: usize) -> u8 {
        // println!("    ; audio_registers[0x{:02X}] ", index);
        self.aud.registers[index]
    }

    fn set_audio_register(&mut self, index: usize, value: u8) {
        // println!("    ; audio_registers[0x{:02X}] = {:02X}", index, value);
        self.aud.registers[index] = value;
    }
}
