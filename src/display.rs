// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// display.rs
//
// 64x32 framebuffer, sprite drawing

use minifb::{Window, WindowOptions};

/// Width of the CHIP-8 screen in pixels.
pub const WIDTH: usize = 64;
/// Height of the CHIP-8 screen in pixels.
pub const HEIGHT: usize = 32;
/// How many window pixels each CHIP-8 pixel takes up, in each direction.
pub const SCALE: usize = 10;

/// The CHIP-8 framebuffer.
pub struct Display {
    /// `WIDTH * HEIGHT` pixels in row-major order. 0 is off, and 0xFFFFFFFF is on.
    pub video: Vec<u32>,
}

impl Display {
    /// Creates a blank screen with every pixel off.
    pub fn new() -> Self {
        Self {
            video: vec![0u32; WIDTH * HEIGHT],
        }
    }
}

/// Opens a window sized `WIDTH * SCALE` by `HEIGHT * SCALE`, and returns it
/// with a matching blank pixel buffer to draw into.
///
/// Panics if the window can't be created.
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

/// Scales the CHIP-8 framebuffer `display` up into the window `buffer`, drawing
/// lit pixels as white and unlit ones as black.
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
