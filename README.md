# Chip-8

A CHIP-8 interpreter written in Rust. It renders the 64×32 CHIP-8 display scaled 10× (a 640×320 window) using [`minifb`](https://crates.io/crates/minifb), running at 60 FPS.

> **Status:** work in progress. The CPU core and a subset of the instruction set are in place; drawing, input, and sound are not yet implemented.

## Requirements

- A Rust toolchain with edition 2024 support (Rust 1.85 or newer)

## Building and running

```sh
cargo run --release
```

Run it from the repository root: `font.txt` and the ROM are loaded by relative path. Press **Esc** to quit.

### Choosing a ROM

The ROM path is currently hardcoded in `src/main.rs`:

```rust
match load_rom(ram, "roms/IBM Logo.ch8") {
```

Bundled ROMs in `roms/`:

- `IBM Logo.ch8`
- `Breakout [Carmelo Cortez, 1979].ch8`

## Project layout

| Path                  | Purpose                                                                 |
| --------------------- | ----------------------------------------------------------------------- |
| `src/main.rs`         | ROM loading, window setup, main fetch/decode/execute loop, timers       |
| `src/cpu.rs`          | `Cpu` state (V0–VF, I, PC, SP, stack, timers), `fetch` and `decode`     |
| `src/instructions.rs` | One function per opcode (`op_00E0`, `op_8xy4`, …)                       |
| `src/memory.rs`       | 4 KB RAM; loads the built-in font                                       |
| `src/display.rs`      | Window creation and scaled framebuffer drawing                          |
| `font.txt`            | Hex font sprite data (0–F, 5 bytes each)                                |
| `roms/`               | Test ROMs                                                               |

## Memory map

| Address         | Contents                                |
| --------------- | --------------------------------------- |
| `0x000`–`0x1FF` | Reserved for the interpreter            |
| `0x050`–`0x09F` | Font sprites                            |
| `0x200`–`0xFFF` | Program ROM and working memory          |

## Instruction support

Implemented:

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

Not yet implemented:

- [ ] `9xy0` SNE Vx, Vy
- [ ] `Annn` LD I, addr
- [ ] `Bnnn` JP V0, addr
- [ ] `Cxkk` RND Vx, byte
- [ ] `Dxyn` DRW (sprite drawing)
- [ ] `Ex9E` / `ExA1` key skips
- [ ] `Fx07`–`Fx65` timers, keys, BCD, memory ops

Other to-dos:

- [ ] Write sprites into the framebuffer (nothing is drawn to the screen until `Dxyn` is in)
- [ ] Keypad input
- [ ] Sound
- [ ] Choose the ROM from the command line

## References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff, *Guide to making a CHIP-8 emulator*](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## License

Copyright (c) 2026 Michael Kestner. All Rights Reserved.
