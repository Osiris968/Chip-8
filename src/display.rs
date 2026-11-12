// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// display.rs
//
// 64x32 framebuffer, sprite drawing

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

pub fn handle_window() {
    let mut window = Window::new(
        "Chip-8 ROM",
        WIDTH * SCALE,
        HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .unwrap();

    let mut buffer: Vec<u32> = vec![0; (WIDTH * SCALE) * (HEIGHT * SCALE)];

    let display = vec![0; WIDTH * HEIGHT];
    window.set_target_fps(60);

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
}
