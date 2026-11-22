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
use crate::display::{HEIGHT, SCALE, WIDTH};
use minifb::{Key, Window};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const ROM_START_ADDRESS: usize = 0x200;
const INSTRUCTIONS_PER_FRAME: usize = 10;

fn load_rom(memory: &mut [u8], rom_path: &str) {
    let Ok(rom_data) = fs::read(rom_path) else {
        eprintln!("Could not read file: {}", rom_path);
        std::process::exit(1);
    };

    // Only 0x200..0xFFF is available for the program.
    let max_size = memory.len() - ROM_START_ADDRESS;
    if rom_data.len() > max_size {
        eprintln!(
            "ROM is too large ({} bytes, max is {} bytes)",
            rom_data.len(),
            max_size
        );
        std::process::exit(1);
    }

    memory[ROM_START_ADDRESS..ROM_START_ADDRESS + rom_data.len()].copy_from_slice(&rom_data);
}

/// Clean up a path typed or dragged into the terminal: strip surrounding
/// whitespace and quotes, and turn escaped spaces ("\ ") back into spaces.
fn clean_path(raw: &str) -> String {
    raw.trim()
        .trim_matches(|c| c == '\'' || c == '"')
        .replace("\\ ", " ")
}

fn choose_rom(memory: &mut [u8]) {
    // Use the path from the command line if one was given.
    if let Some(arg) = env::args().nth(1) {
        let path = clean_path(&arg);

        if !Path::new(&path).is_file() {
            eprintln!("ROM {} does not exist!", path);
            std::process::exit(1);
        }

        load_rom(memory, &path);
        return;
    }

    // Otherwise, keep asking until we get a valid path.
    loop {
        print!("Please enter the path to your desired ROM: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nNo input, exiting.");
                std::process::exit(0);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read input: {e}");
                std::process::exit(1);
            }
        }

        let path = clean_path(&input);

        if path.is_empty() {
            continue;
        }

        if !Path::new(&path).is_file() {
            eprintln!("ROM {} does not exist! Try again.", path);
            continue;
        }

        load_rom(memory, &path);
        break;
    }
}

fn main() -> io::Result<()> {
    let mut emulator = Chip8::new();

    choose_rom(emulator.cpu.ram.get_mut_vec());

    let (mut window, mut buffer): (Window, Vec<u32>) = display::create_window();
    window.set_target_fps(60);

    // Main loop.
    'main: while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            if !emulator.step() {
                break 'main;
            }
        }
        emulator.tick_timers();

        display::draw_window(&emulator.display.video, &mut buffer);
        if let Err(e) = window.update_with_buffer(&buffer, WIDTH * SCALE, HEIGHT * SCALE) {
            eprintln!("Error writing to window buffer: {}", e);
            break;
        }
    }

    Ok(())
}
