use super::GameBoy;

pub struct AudioData {
    registers: [u8; 0x2F],
}

impl AudioData {
    pub fn new() -> Self {
        Self {
            registers: [0; 0x2F],
        }
    }
}

pub trait AudioController {
    fn audio_register(&self, index: usize) -> u8;
    fn set_audio_register(&mut self, index: usize, value: u8);
}

impl AudioController for GameBoy {
    fn audio_register(&self, index: usize) -> u8 {
        println!("    ; audio_registers[${:02x}] ", index);
        self.aud.registers[index]
    }

    fn set_audio_register(&mut self, index: usize, value: u8) {
        println!("    ; audio_registers[${:02x}] = ${:02x}", index, value);
        self.aud.registers[index] = value;
    }
}
