// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

mod cpu;
mod display;
mod instructions;
mod memory;
mod test;

use minifb::{Key, Window};
use std::fs;
use std::io;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;
const ROM_START_ADDRESS: usize = 0x200;

fn load_rom(memory: &mut [u8], rom_path: &str) -> io::Result<()> {
    let rom_data = fs::read(rom_path)?;

    for (i, &byte) in rom_data.iter().enumerate() {
        memory[ROM_START_ADDRESS + i] = byte;
    }

    Ok(())
}

#[allow(unused)]
fn print_memory(ram: &[u8]) {
    for (index, byte) in ram.iter().enumerate() {
        println!("Address {}: {}", index, byte);
    }
}

#[allow(clippy::all)]
fn decode(opcode: u16, cpu: &mut cpu::Cpu) {
    match opcode {
        0x00EE => {
            instructions::op_return(cpu);
        }
        _ => {}
    }
}

fn main() -> io::Result<()> {
    // Creating memory vector and loading rom into memory.
    let mut ram = memory::Memory::new();
    ram.load_font();
    let ram = ram.get_mut_vec();

    let mut cpu = cpu::Cpu::new();

    match load_rom(ram, "roms/IBM Logo.ch8") {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Couldn't load rom: {}", e);
            return Ok(());
        }
    }

    let (mut window, mut buffer): (Window, Vec<u32>) = display::create_window();

    let display = vec![0; WIDTH * HEIGHT];
    window.set_target_fps(60);

    // NOTE: Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        display::draw_window(&display, &mut buffer);

        if cpu.pc == ram.len() {
            break; // Program counter is out of bounds!
        }

        // Fetch instruction
        let opcode = cpu::Cpu::fetch(ram[cpu.pc], ram[cpu.pc + 1]);

        // Increment the pc before we execute anything.
        cpu.pc += 2;

        match opcode {
            0x00E0 => {
                instructions::op_clear_screen(&mut buffer);
            }
            _ => decode(opcode, &mut cpu),
        }

        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }
        if cpu.sound_timer > 0 {
            cpu.sound_timer -= 1;
        }

        match window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Error writing to window buffer: {}", e);
                break;
            }
        }
    }

    Ok(())
}
