#![allow(unused)]

// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// cpu.rs
//
// Registers, fetch/decode/execute

use crate::instructions;

pub struct Cpu {
    pub v: [u8; 16], // V0..VF (VF is the flag register)
    pub i: u16,      // index register
    pub pc: u16,     // starts at 0x200
    pub sp: usize,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn fetch(pc1: u8, pc2: u8) -> u16 {
        // Shift pc1 8 bits to the left and add pc2 to the now empty 8 bits on the right.
        (pc1 as u16) << 8 | (pc2 as u16)
    }

    pub fn decode(&mut self, opcode: u16) {
        match opcode {
            0x00EE => {
                instructions::op_00EE(self);
            }
            opcode if (opcode >> 12) == 1 => {
                instructions::op_1nnn(opcode, self);
            }
            opcode if (opcode >> 12) == 2 => {
                instructions::op_2nnn(opcode, self);
            }
            opcode if (opcode >> 12) == 3 => {
                instructions::op_3xkk(opcode, self);
            }
            opcode if (opcode >> 12) == 4 => {
                instructions::op_4xkk(opcode, self);
            }
            opcode if (opcode >> 12) == 5 => {
                instructions::op_5xy0(opcode, self);
            }
            opcode if (opcode >> 12) == 6 => {
                instructions::op_6xkk(opcode, self);
            }
            opcode if (opcode >> 12) == 7 => {
                instructions::op_7xkk(opcode, self);
            }
            // Match all 0x8nnn instructions.
            opcode if (opcode >> 12) == 8 => match opcode {
                opcode if (opcode & 0xF) == 0 => {
                    instructions::op_8xy0(opcode, self);
                }
                opcode if (opcode & 0xF) == 1 => {
                    instructions::op_8xy1(opcode, self);
                }
                opcode if (opcode & 0xF) == 2 => {
                    instructions::op_8xy2(opcode, self);
                }
                opcode if (opcode & 0xF) == 3 => {
                    instructions::op_8xy3(opcode, self);
                }
                opcode if (opcode & 0xF) == 4 => {
                    instructions::op_8xy4(opcode, self);
                }
                opcode if (opcode & 0xF) == 5 => {
                    instructions::op_8xy5(opcode, self);
                }
                opcode if (opcode & 0xF) == 6 => {
                    instructions::op_8xy6(opcode, self);
                }
                opcode if (opcode & 0xF) == 7 => {
                    instructions::op_8xy7(opcode, self);
                }
                opcode if (opcode & 0xF) == 0xE => {
                    instructions::op_8xyE(opcode, self);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
