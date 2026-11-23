// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// cpu.rs
//
// Registers, fetch/decode/execute

use crate::{instructions, memory::Memory};

/// CHIP-8 processor state: registers, timers, call stack, and RAM.
pub struct Cpu {
    /// General-purpose registers V0..VF. VF doubles as the flag register for
    /// carry, borrow, shifted-out bits, and sprite collisions.
    pub v: [u8; 16],
    /// Index register `I`, used as a memory address by draw and load/store opcodes.
    pub i: u16,
    /// Program counter: the address of the next instruction. Starts at 0x200.
    pub pc: u16,
    /// Stack pointer: the number of return addresses currently on `stack`.
    pub sp: usize,
    /// Return addresses for subroutine calls (up to 16 levels deep).
    pub stack: [u16; 16],
    /// Counts down at 60 Hz while non-zero. Programs read it for timing.
    pub delay_timer: u8,
    /// Counts down at 60 Hz while non-zero. The buzzer should sound while it's above zero.
    pub sound_timer: u8,
    /// 4 KB of RAM, with the font at 0x50 and the program at 0x200.
    pub ram: Memory,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    /// Creates a CPU with zeroed registers, `pc` at 0x200, and RAM with the font loaded.
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            ram: Memory::new(),
        }
    }

    /// Reads the two-byte, big-endian opcode at `pc` without advancing it.
    ///
    /// Returns `None` if `pc` is too close to the end of RAM to read a full opcode.
    pub fn fetch(&self) -> Option<u16> {
        let pc = self.pc as usize;

        if pc + 1 >= self.ram.data.len() {
            return None;
        }

        let high = self.ram.data[pc];
        let low = self.ram.data[pc + 1];

        // Shift the high byte 8 bits to the left
        // and put the low byte in the right 8 bits.
        Some((high as u16) << 8 | (low as u16))
    }

    /// Decodes and executes an opcode that only touches CPU state.
    ///
    /// Opcodes that need the display or keypad are handled by
    /// [`Chip8::step`](crate::chip_8::Chip8::step) instead. Unknown opcodes,
    /// including `0nnn` (SYS), are ignored.
    pub fn decode(&mut self, opcode: u16) {
        match opcode {
            0x00EE => {
                instructions::op_00EE(self);
            }
            opcode if opcode >> 12 == 1 => {
                instructions::op_1nnn(opcode, self);
            }
            opcode if opcode >> 12 == 2 => {
                instructions::op_2nnn(opcode, self);
            }
            opcode if opcode >> 12 == 3 => {
                instructions::op_3xkk(opcode, self);
            }
            opcode if opcode >> 12 == 4 => {
                instructions::op_4xkk(opcode, self);
            }
            opcode if opcode >> 12 == 5 => {
                instructions::op_5xy0(opcode, self);
            }
            opcode if opcode >> 12 == 6 => {
                instructions::op_6xkk(opcode, self);
            }
            opcode if opcode >> 12 == 7 => {
                instructions::op_7xkk(opcode, self);
            }
            // Match all 0x8nnn instructions.
            opcode if opcode >> 12 == 8 => match opcode {
                opcode if opcode & 0xF == 0 => {
                    instructions::op_8xy0(opcode, self);
                }
                opcode if opcode & 0xF == 1 => {
                    instructions::op_8xy1(opcode, self);
                }
                opcode if opcode & 0xF == 2 => {
                    instructions::op_8xy2(opcode, self);
                }
                opcode if opcode & 0xF == 3 => {
                    instructions::op_8xy3(opcode, self);
                }
                opcode if opcode & 0xF == 4 => {
                    instructions::op_8xy4(opcode, self);
                }
                opcode if opcode & 0xF == 5 => {
                    instructions::op_8xy5(opcode, self);
                }
                opcode if opcode & 0xF == 6 => {
                    instructions::op_8xy6(opcode, self);
                }
                opcode if opcode & 0xF == 7 => {
                    instructions::op_8xy7(opcode, self);
                }
                opcode if opcode & 0xF == 0xE => {
                    instructions::op_8xyE(opcode, self);
                }
                _ => {}
            },
            opcode if opcode >> 12 == 9 => {
                instructions::op_9xy0(opcode, self);
            }
            opcode if opcode >> 12 == 0xA => {
                instructions::op_Annn(opcode, self);
            }
            opcode if opcode >> 12 == 0xB => {
                instructions::op_Bnnn(opcode, self);
            }
            opcode if opcode >> 12 == 0xC => {
                instructions::op_Cxkk(opcode, self);
            }
            // Match all 0xFnnn instructions.
            opcode if opcode >> 12 == 0xF => match opcode {
                opcode if opcode & 0xF0FF == 0xF007 => {
                    instructions::op_Fx07(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF015 => {
                    instructions::op_Fx15(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF018 => {
                    instructions::op_Fx18(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF01E => {
                    instructions::op_Fx1E(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF029 => {
                    instructions::op_Fx29(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF033 => {
                    instructions::op_Fx33(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF055 => {
                    instructions::op_Fx55(opcode, self);
                }
                opcode if opcode & 0xF0FF == 0xF065 => {
                    instructions::op_Fx65(opcode, self);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
