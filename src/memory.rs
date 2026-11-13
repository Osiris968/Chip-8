// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// memory.rs
//
// 4kB RAM, Font Loading, ROM Loading
//
// Leave empty space (except for the fonts) from 0x0000 to 0x01FF,
// as this is where CHIP-8 expects the interpreter to live.
//
// Do I need a table?

use std::fs;

// pub struct AddressPointer {
//     start_pos: u8,
//     length: u8,
// }

pub struct Memory {
    data: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = Self {
            data: vec![0u8; 4096],
        };

        ram.load_font();

        ram
    }

    // Returns a mutable reference bound to the lifetime of the struct.
    pub fn get_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    // Load the font into addresses 0x50 - 0x9F. (80 values)
    pub fn load_font(&mut self) {
        // Load font values into string.
        let content = match fs::read_to_string("font.txt") {
            Ok(s) => s,
            Err(_) => {
                return;
            }
        };

        // Put the font value string into a vector by splitting up the spaces and converting to u8.
        let font_values: Vec<u8> = content
            .split_whitespace()
            .map(|s| s.trim_start_matches("0x"))
            .map(|s| u8::from_str_radix(s, 16))
            .collect::<Result<Vec<u8>, _>>()
            // :)
            .unwrap();

        // Copy the font values into memory at addresses 0x0050 to 0x009F.
        self.data[0x0050..0x0050 + font_values.len()].copy_from_slice(&font_values);
    }
}
