#![allow(unused)]

// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// main.rs
//
// Frontend: window, input, timing loop

use chip8::memory::Memory;
use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::io;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;
const MEMORY_SIZE: usize = 4096;
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
    let mut memory = Memory::new();
    memory.load_font();
    let ram = memory.get_mut_vec();

    let mut display = vec![0; WIDTH * HEIGHT];
    let mut _pc: usize = ROM_START_ADDRESS;

    let mut window = Window::new(
        "Chip-8 ROM",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; (WIDTH * SCALE) * (HEIGHT * SCALE)];

    load_rom(ram, "roms/IBM Logo.ch8")?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        window
            .update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE)
            .unwrap();
    }

    Ok(())
}
