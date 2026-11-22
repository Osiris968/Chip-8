use crate::display::{self, Display, HEIGHT, SCALE, WIDTH};
use crate::{cpu::Cpu, instructions};

use minifb::Window;

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

    pub fn step(&mut self, window: &mut Window, buffer: &mut [u32]) -> bool {
        if self.cpu.pc as usize + 1 >= self.cpu.ram.data.len() {
            return false; // Program counter is out of bounds!
        }

        // Returns if PC is out of bounds.
        let Some(opcode) = self.cpu.fetch() else {
            return false;
        };

        // Increment the pc before we execute anything.
        self.cpu.pc += 2;

        // Decode instruction.
        match opcode {
            0x00E0 => {
                instructions::op_00E0(&mut self.display.video);
            }
            opcode if (opcode >> 12) == 0xD => {
                instructions::op_Dxyn(opcode, &mut self.cpu, &mut self.display);
            }
            _ => Cpu::decode(&mut self.cpu, opcode),
        }

        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1;
        }

        display::draw_window(&self.display.video, buffer);
        match window.update_with_buffer(buffer, WIDTH * SCALE, HEIGHT * SCALE) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Error writing to window buffer: {}", e);
                return false;
            }
        }
        true
    }
}
