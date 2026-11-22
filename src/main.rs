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

fn main() -> io::Result<()> {
    let mut emulator = Chip8::new();
    // let mut cpu = cpu::Cpu::new();

    match load_rom(emulator.cpu.ram.get_mut_vec(), "roms/IBM Logo.ch8") {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Couldn't load rom: {}", e);
            return Ok(());
        }
    }

    let (mut window, mut buffer): (Window, Vec<u32>) = display::create_window();
    window.set_target_fps(60);
    let chip_8::Chip8 { cpu, display } = &mut emulator;

    // NOTE: Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if cpu.pc as usize + 1 >= cpu.ram.data.len() {
            break; // Program counter is out of bounds!
        }

        // Fetch instruction. Breaks if pc if out of bounds.
        let Some(opcode) = cpu.fetch() else {
            break;
        };

        // Increment the pc before we execute anything.
        cpu.pc += 2;

        // Decode instruction.
        match opcode {
            0x00E0 => {
                instructions::op_00E0(&mut display.video);
            }
            opcode if (opcode >> 12) == 0xD => {
                instructions::op_Dxyn(opcode, cpu, display);
            }
            _ => cpu::Cpu::decode(cpu, opcode),
        }

        if cpu.delay_timer > 0 {
            cpu.delay_timer -= 1;
        }
        if cpu.sound_timer > 0 {
            cpu.sound_timer -= 1;
        }

        display::draw_window(&display.video, &mut buffer);
        match window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE) {
            Ok(()) => (),
            Err(e) => {
                eprintln!("Error writing to window buffer: {}", e);
                break;
            }
        }
    } // End main loop.

    Ok(())
}
