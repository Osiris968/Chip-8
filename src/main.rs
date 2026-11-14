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

    let instruction: (u8, u8) = (
        match ram.get(cpu.pc) {
            Some(val) => val.to_owned(),
            None => {
                eprintln!("Could not find value");
                0
            }
        },
        match ram.get(cpu.pc + 1) {
            Some(val) => val.to_owned(),
            None => {
                eprintln!("Could not find value");
                0
            }
        },
    );
    // Not as safe, but 14 less lines lmfao.
    // let instruction: (u8, u8) = (ram[cpu.pc], ram[cpu.pc + 1]);
    println!("{:X?}", instruction);

    let _instr: u16 = (instruction.0 as u16) << 8 | (instruction.1 as u16);

    Ok(())
}
