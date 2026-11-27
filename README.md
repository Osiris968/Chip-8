# Chip-8

A CHIP-8 interpreter written in Rust. It renders the 64×32 CHIP-8 display scaled 10× (a 640×320 window) using [`minifb`](https://crates.io/crates/minifb), running at 60 FPS, and plays the buzzer through [`rodio`](https://crates.io/crates/rodio).

> **Status:** work in progress. The full original CHIP-8 instruction set, keypad input, and sound are in place.

## Requirements

- A Rust toolchain with edition 2024 support (Rust 1.85 or newer)
- An audio output device: the emulator exits at startup if it can't open one
- On Linux, the ALSA development headers and `pkg-config` (for example, `sudo apt install libasound2-dev pkg-config` on Debian/Ubuntu)

## Building and running

```sh
cargo run --release -- "roms/IBM Logo.ch8"
```

Run it from the repository root: `font.txt` is loaded by relative path, and relative ROM paths are resolved from the current directory. Press **Esc** to quit.

The emulator runs 10 instructions per frame at 60 FPS (about 600 instructions per second), and the delay and sound timers count down once per frame. While the sound timer is above zero, a quiet 440 Hz square-wave tone plays.

### Choosing a ROM

Pass the ROM path as the first argument (after `--` when using `cargo run`), as shown above. If you don't give one, the emulator asks for a path:

```text
Please enter the path to your desired ROM: roms/Maze.ch8
```

If the path doesn't exist, it asks again. You can also drag a file into the terminal: quotes and escaped spaces (`IBM\ Logo.ch8`) are handled. ROMs must fit in the 3584 bytes from `0x200` to `0xFFF`.

Bundled ROMs in `roms/`:

- `IBM Logo.ch8`
- `Breakout [Carmelo Cortez, 1979].ch8`
- `Maze.ch8`
- `Particle Demo.ch8`
- `Stars.ch8`
- `Trip8 Demo.ch8`
- `Zero Demo.ch8`

### Controls

The 16-key CHIP-8 keypad is mapped to the left side of the keyboard:

```text
CHIP-8 keypad      Keyboard
1 2 3 C            1 2 3 4
4 5 6 D            Q W E R
7 8 9 E            A S D F
A 0 B F            Z X C V
```

Each game picks its own keys. For example, Breakout moves the paddle with CHIP-8 keys 4 and 6 (**Q** and **E**), and you start each ball by pressing any key.

## Project layout

| Path                  | Purpose                                                                 |
| --------------------- | ----------------------------------------------------------------------- |
| `src/main.rs`         | ROM loading, window setup, keyboard mapping, audio, main loop           |
| `src/chip_8.rs`       | `Chip8` tying together the `Cpu`, `Display`, and keypad; `step` runs one instruction (and handles the display and keypad opcodes); `tick_timers`; `is_beeping` |
| `src/cpu.rs`          | `Cpu` state (V0–VF, I, PC, SP, stack, timers, RAM), `fetch` and `decode` for the CPU-only opcodes |
| `src/instructions.rs` | One function per opcode (`op_00E0`, `op_8xy4`, …)                       |
| `src/memory.rs`       | 4 KB RAM; loads the built-in font                                       |
| `src/display.rs`      | `Display` 64×32 framebuffer, window creation, scaled drawing            |
| `font.txt`            | Hex font sprite data (0–F, 5 bytes each)                                |
| `roms/`               | Test ROMs                                                               |

## Memory map

| Address         | Contents                                |
| --------------- | --------------------------------------- |
| `0x000`–`0x1FF` | Reserved for the interpreter            |
| `0x050`–`0x09F` | Font sprites                            |
| `0x200`–`0xFFF` | Program ROM and working memory          |

## Instruction support

Every original CHIP-8 instruction except `0nnn` is implemented:

- [x] `00E0` CLS
- [x] `00EE` RET
- [x] `1nnn` JP addr
- [x] `2nnn` CALL addr
- [x] `3xkk` SE Vx, byte
- [x] `4xkk` SNE Vx, byte
- [x] `5xy0` SE Vx, Vy
- [x] `6xkk` LD Vx, byte
- [x] `7xkk` ADD Vx, byte
- [x] `8xy0`–`8xy7`, `8xyE` arithmetic and logic
- [x] `9xy0` SNE Vx, Vy
- [x] `Annn` LD I, addr
- [x] `Bnnn` JP V0, addr
- [x] `Cxkk` RND Vx, byte
- [x] `Dxyn` DRW (sprite drawing; the start position wraps, and the sprite clips at the screen edges)
- [x] `Ex9E` / `ExA1` SKP / SKNP (skip if a key is / isn't pressed)
- [x] `Fx07` LD Vx, DT
- [x] `Fx0A` LD Vx, K (wait for a key press)
- [x] `Fx15` / `Fx18` LD DT, Vx / LD ST, Vx
- [x] `Fx1E` ADD I, Vx
- [x] `Fx29` LD F, Vx (font sprite address)
- [x] `Fx33` LD B, Vx (BCD)
- [x] `Fx55` / `Fx65` store / load V0–Vx

`0nnn` (SYS), which called machine code on the original hardware, is ignored, as in most modern interpreters. Unknown opcodes are skipped.

### Behavior notes

Where interpreters disagree, this one follows the common modern behavior:

- `8xy6` / `8xyE` shift Vx in place. The original COSMAC VIP shifted Vy into Vx.
- `Fx55` / `Fx65` leave I unchanged. The COSMAC VIP advanced it.
- `Fx0A` fires when a key is pressed. The COSMAC VIP waited for the key to be released.
- In `8xy4`–`8xyE`, VF is written after the result, so the flag wins when x is F.

Bad ROM data prints a warning to stderr instead of crashing:

- Returning with an empty stack, or calling with a full one (16 levels), skips the instruction.
- `Fx33`, `Fx55`, and `Fx65` skip the instruction if their memory range runs past the end of RAM. `Dxyn` stops drawing at the end of RAM.

### Known ROM quirks

In the bundled Breakout, bricks sometimes reappear where the ball has already broken through. That's how the 1979 program works, not an emulator bug: it erases a brick by XORing a 4-pixel strip at the ball's position, which turns pixels cleared by an earlier hit back on.

## To-dos

- [x] Keypad input
- [x] Choose the ROM from the command line or a prompt
- [x] Sound

## References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff, *Guide to making a CHIP-8 emulator*](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## License

Copyright (c) 2026 Michael Kestner. All Rights Reserved.
