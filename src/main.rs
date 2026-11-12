// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

// use crate::cpu::Cpu;
use crate::memory::Memory;
use std::fs;
use std::io;

const ROM_START_ADDRESS: usize = 0x200;

mod cpu;
mod display;
mod memory;

fn load_rom(memory: &mut [u8], rom_path: &str) -> io::Result<()> {
    let rom_data = fs::read(rom_path)?;

    for (i, &byte) in rom_data.iter().enumerate() {
        memory[ROM_START_ADDRESS + i] = byte;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Creating memory vector and loading rom into memory.
    let mut memory = Memory::new();
    memory.load_font();
    let ram = memory.get_mut_vec();

    let _cpu = cpu::Cpu::new();

    display::handle_window();

    load_rom(ram, "roms/IBM Logo.ch8")?;

    Ok(())
}
