// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

mod chip_8;
mod cpu;
mod display;
mod instructions;
mod memory;

use crate::chip_8::Chip8;
use minifb::{Key, Window};
use std::fs;
use std::io;

const ROM_START_ADDRESS: usize = 0x200;

fn load_rom(memory: &mut [u8], rom_path: &str) -> io::Result<()> {
    let rom_data = fs::read(rom_path)?;

    for (i, &byte) in rom_data.iter().enumerate() {
        memory[ROM_START_ADDRESS + i] = byte;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut emulator = Chip8::new();

    match load_rom(emulator.cpu.ram.get_mut_vec(), "roms/IBM Logo.ch8") {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Couldn't load rom: {}", e);
            return Ok(());
        }
    }

    let (mut window, mut buffer): (Window, Vec<u32>) = display::create_window();
    window.set_target_fps(60);

    // NOTE: Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip_8::Chip8::step(&mut emulator, &mut window, &mut buffer);
    } // End main loop.

    Ok(())
}
