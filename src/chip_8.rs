use crate::display::Display;
use crate::{cpu::Cpu, instructions};

/// The whole CHIP-8 machine: the CPU (with its RAM), the screen, and the keypad.
pub struct Chip8 {
    /// Registers, timers, stack, and RAM.
    pub cpu: Cpu,
    /// The 64x32 framebuffer that `00E0` and `Dxyn` draw into.
    pub display: Display,
    /// Which of the 16 keys (0x0..=0xF) are held down right now.
    /// The frontend should update this once per frame.
    pub keypad: [bool; 16],
}

impl Chip8 {
    /// Creates a machine with a fresh CPU, a blank screen, and no keys pressed.
    /// No ROM is loaded; copy one into `cpu.ram` at 0x200 before stepping.
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            display: Display::new(),
            keypad: [false; 16],
        }
    }

    /// Fetches, decodes, and executes one instruction.
    ///
    /// Opcodes that need the display or keypad (`00E0`, `Dxyn`, `Ex9E`,
    /// `ExA1`, `Fx0A`) are handled here; everything else goes to
    /// [`Cpu::decode`].
    ///
    /// Returns `false` if the program counter has run off the end of RAM,
    /// meaning the emulator should stop.
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

    /// Counts the delay and sound timers down by one, stopping at zero.
    /// Call this at 60 Hz (once per frame).
    pub fn tick_timers(&mut self) {
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
        if self.cpu.sound_timer > 0 {
            self.cpu.sound_timer -= 1;
        }
    }
}
