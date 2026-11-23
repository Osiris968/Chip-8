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

/// The CHIP-8's 4 KB of RAM.
pub struct Memory {
    /// 4096 bytes. The font lives at 0x50..0xA0 and programs load at 0x200.
    pub data: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates zeroed RAM and loads the font into it.
    pub fn new() -> Self {
        let mut ram = Self {
            data: vec![0u8; 4096],
        };

        ram.load_font();

        ram
    }

    /// Returns a mutable reference to the raw RAM bytes, e.g. for loading a ROM.
    pub fn get_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Loads the font into addresses 0x50 - 0x9F (80 values: 16 glyphs of 5 bytes).
    ///
    /// Reads whitespace-separated hex bytes from `font.txt` in the current
    /// directory. If the file can't be read, the font is left blank; if it
    /// contains something that isn't a hex byte, this panics.
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
