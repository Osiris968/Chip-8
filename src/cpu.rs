#![allow(unused)]

// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// cpu.rs
//
// Registers, fetch/decode/execute

pub struct Cpu {
    pub v: [u8; 16], // V0..VF (VF is the flag register)
    pub i: u16,      // index register
    pub pc: usize,   // starts at 0x200
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

    pub fn decode(instruction: u16) {}
}
