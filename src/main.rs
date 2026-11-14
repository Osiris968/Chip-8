// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

mod cpu;
mod display;
mod memory;

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

#[allow(unused)]
fn print_memory(ram: &mut [u8]) {
    for (index, byte) in ram.iter().enumerate() {
        println!("Address {}: {}", index, byte);
    }
}

fn main() -> io::Result<()> {
    // Creating memory vector and loading rom into memory.
    let mut ram = memory::Memory::new();
    ram.load_font();
    let ram = ram.get_mut_vec();

    let cpu = cpu::Cpu::new();

    match load_rom(ram, "roms/IBM Logo.ch8") {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Couldn't load rom: {}", e);
            return Ok(());
        }
    }

    // TODO: Ignore the display until I get the cpu under control.
    // let (window, buffer) = display::create_window();

    // This holds the whole program in the loop until the window closes.
    // I probably want to change this to have the main loop in this function.
    // display::handle_window(window, buffer);

    if cpu.pc == ram.len() {
        // I probably want this to break when I have the main loop ready.
    }
    let instruction = cpu::Cpu::fetch(ram[cpu.pc], ram[cpu.pc + 1]);

    println!("{:?}", instruction);

    // print_memory(ram);

    Ok(())
}
