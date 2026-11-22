# Chip-8

A CHIP-8 interpreter written in Rust. It renders the 64×32 CHIP-8 display scaled 10× (a 640×320 window) using [`minifb`](https://crates.io/crates/minifb), running at 60 FPS.

> **Status:** work in progress. The CPU core, sprite drawing, and most of the instruction set are in place, and the IBM Logo ROM renders correctly. Input, the `Ex__`/`Fx__` opcodes, and sound are not yet implemented.

## Requirements

- A Rust toolchain with edition 2024 support (Rust 1.85 or newer)

## Building and running

```sh
cargo run --release -- "roms/IBM Logo.ch8"
```

Run it from the repository root: `font.txt` is loaded by relative path, and relative ROM paths are resolved from the current directory. Press **Esc** to quit.

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

## Project layout

| Path                  | Purpose                                                                 |
| --------------------- | ----------------------------------------------------------------------- |
| `src/main.rs`         | ROM loading, window setup, main fetch/decode/execute loop, timers       |
| `src/chip_8.rs`       | `Chip8` struct tying together the `Cpu` and `Display`                   |
| `src/cpu.rs`          | `Cpu` state (V0–VF, I, PC, SP, stack, timers), `fetch` and `decode`     |
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
- [x] `9xy0` SNE Vx, Vy
- [x] `Annn` LD I, addr
- [x] `Bnnn` JP V0, addr
- [x] `Cxkk` RND Vx, byte
- [x] `Dxyn` DRW (sprite drawing; the start position wraps, and the sprite clips at the screen edges)

Not yet implemented:

- [ ] `Ex9E` / `ExA1` key skips
- [ ] `Fx07`–`Fx65` timers, keys, BCD, memory ops

Other to-dos:

- [ ] Keypad input
- [ ] Sound
- [x] Choose the ROM from the command line or a prompt

## References

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff, *Guide to making a CHIP-8 emulator*](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

## License

Copyright (c) 2026 Michael Kestner. All Rights Reserved.
