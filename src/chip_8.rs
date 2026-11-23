use crate::display::Display;
use crate::{cpu::Cpu, instructions};

pub struct Chip8 {
    pub cpu: Cpu,
    pub display: Display,
    pub keypad: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: [false; 16],
        }
    }

    pub fn step(&mut self) -> bool {
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
            opcode if opcode >> 12 == 0xD => {
                instructions::op_Dxyn(opcode, &mut self.cpu, &mut self.display);
            }
            opcode if opcode & 0xF0FF == 0xE09E => {
                instructions::op_Ex9E(opcode, &mut self.cpu, &self.keypad);
            }
            opcode if opcode & 0xF0FF == 0xE0A1 => {
                instructions::op_ExA1(opcode, &mut self.cpu, &self.keypad);
            }
            opcode if opcode & 0xF0FF == 0xF00A => {
                instructions::op_Fx0A(opcode, &mut self.cpu, &self.keypad);
            }
            _ => Cpu::decode(&mut self.cpu, opcode),
        }

        true
    }

    pub fn tick_timers(&mut self) {
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1;
        }
    }
}
