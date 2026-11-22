use crate::{cpu::Cpu, display::Display};

pub struct Chip8 {
    pub cpu: Cpu,
    pub display: Display,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            display: Display::new(),
        }
    }
}
