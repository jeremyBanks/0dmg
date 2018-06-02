use super::GameBoy;
pub struct AudioData;

impl AudioData {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait AudioController {}

impl AudioController for GameBoy {}
