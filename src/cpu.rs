// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// cpu.rs
//
// Registers, fetch/decode/execute

pub struct Cpu {
    pub v: [u8; 16], // V0..VF (VF is the flag register)
    pub i: u16,      // index register
    pub pc: u16,     // starts at 0x200
    pub sp: usize,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}
