// Copyright (c) 2026 Michael Kestner. All Rights Reserved.
// display.rs
//
// 64x32 framebuffer, sprite drawing

use minifb::{Key, Window, WindowOptions};

pub fn create_window() {
    const W: usize = 64;
    const H: usize = 32;

    let mut window = Window::new("test", W, H, WindowOptions::default())
        .expect("could not open window");

    let buffer = vec![0u32; W * H];

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, W, H).unwrap();
    }
}
