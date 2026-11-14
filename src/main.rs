// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

mod cpu;
mod display;
mod memory;
mod test;

use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::io;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;
const ROM_START_ADDRESS: usize = 0x200;

pub fn create_window() -> (Window, Vec<u32>) {
    let window = Window::new(
        "Chip-8 ROM",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap();

    let buffer: Vec<u32> = vec![0; (WIDTH * SCALE) * (HEIGHT * SCALE)];

    (window, buffer)
}

fn draw_window(display: &[i32], buffer: &mut [u32]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = display[y * WIDTH + x];
            let color = if pixel != 0 { 0x00FFFFFF } else { 0x00000000 };

            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let wx = x * SCALE + dx;
                    let wy = y * SCALE + dy;
                    buffer[wy * (WIDTH * SCALE) + wx] = color;
                }
            }
        }
    }
}

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

    let (mut window, mut buffer): (Window, Vec<u32>) = create_window();

    let display = vec![0; WIDTH * HEIGHT];
    window.set_target_fps(60);

    // NOTE: Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_window(&display, &mut buffer);

        if cpu.pc == ram.len() {
            break; // Program counter is out of bounds!
        }

        let _instruction = cpu::Cpu::fetch(ram[cpu.pc], ram[cpu.pc + 1]);

        window
            .update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)
            .unwrap();
    }

    // print_memory(ram);

    Ok(())
}
