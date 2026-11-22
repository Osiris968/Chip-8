// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// display.rs
//
// 64x32 framebuffer, sprite drawing

use minifb::{Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

pub struct Display {
    pub video: Vec<u32>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            video: vec![0u32; WIDTH * HEIGHT],
        }
    }
}

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

pub fn draw_window(display: &[u32], buffer: &mut [u32]) {
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
