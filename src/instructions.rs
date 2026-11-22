#![allow(unused)]

use crate::{cpu, display};
use rand::RngExt;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

/// Clear the display.
#[allow(non_snake_case)]
pub fn op_00E0(buffer: &mut [u32]) {
    buffer.fill(0);
}

/// Return from a subroutine.
#[allow(non_snake_case)]
pub fn op_00EE(cpu: &mut cpu::Cpu) {
    cpu.sp -= 1;

    cpu.pc = cpu.stack[cpu.sp];
}

/// Jump to location nnn.
pub fn op_1nnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = (opcode & 0x0FFF);
    cpu.pc = address;
}

/// Call subroutine at nnn.
pub fn op_2nnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = (opcode & 0x0FFF);

    cpu.stack[cpu.sp] = cpu.pc;
    cpu.sp += 1;

    cpu.pc = address;
}

/// Skip next instruction if Vx = kk.
pub fn op_3xkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    if cpu.v[vx as usize] == byte {
        cpu.pc += 2;
    }
}

/// Skip next instruction if Vx != kk.
pub fn op_4xkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    if cpu.v[vx as usize] != byte {
        cpu.pc += 2;
    }
}

/// Skip next instruction if Vx = Vy.
pub fn op_5xy0(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    if cpu.v[vx as usize] == cpu.v[vy as usize] {
        cpu.pc += 2;
    }
}

/// Set Vx = kk.
pub fn op_6xkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    cpu.v[vx as usize] = byte;
}

/// Set Vx = Vx + kk.
pub fn op_7xkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    cpu.v[vx as usize] = cpu.v[vx as usize].wrapping_add(byte);
}

/// Set Vx = Vy.
pub fn op_8xy0(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    cpu.v[vx as usize] = cpu.v[vy as usize];
}

/// Set Vx = Vx OR Vy.
pub fn op_8xy1(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    cpu.v[vx as usize] |= cpu.v[vy as usize];
}

/// Set Vx = Vx AND Vy.
pub fn op_8xy2(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    cpu.v[vx as usize] &= cpu.v[vy as usize];
}

/// Set Vx = Vx XOR Vy.
pub fn op_8xy3(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    cpu.v[vx as usize] ^= cpu.v[vy as usize];
}

/// Set Vx = Vx + Vy, set VF = carry.
/// The values of Vx and Vy are added together. If the result is greater
/// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8
/// bits of the result are kept, and stored in Vx.
pub fn op_8xy4(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    let sum: u16 = (cpu.v[vx as usize]) as u16 + (cpu.v[vy as usize]) as u16;

    if sum > 255 {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[vx as usize] = (sum & 0xFF) as u8;
}

/// Set Vx = Vx - Vy, set VF = NOT borrow.
/// If Vx > Vy, then VF is set to 1, otherwise 0.
/// Then Vy is subtracted from Vx, and the results stored in Vx.
pub fn op_8xy5(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    if cpu.v[vx as usize] > cpu.v[vy as usize] {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[vx as usize] = cpu.v[vx as usize].wrapping_sub(cpu.v[vy as usize]);
}

/// Set Vx = Vx SHR 1.
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
/// Then Vx is divided by 2.
pub fn op_8xy6(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;

    // Save the Least Significant Bit in VF.
    cpu.v[0xF] = (cpu.v[vx as usize] & 0x1);

    cpu.v[vx as usize] >>= 1;
}

/// Set Vx = Vy - Vx, set VF = NOT borrow.
/// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
/// and the results stored in Vx.
pub fn op_8xy7(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    if cpu.v[vy as usize] > cpu.v[vx as usize] {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    cpu.v[vx as usize] = cpu.v[vy as usize].wrapping_sub(cpu.v[vx as usize]);
}

/// Set Vx = Vx SHL 1
/// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
/// Then Vx is multiplied by 2.
#[allow(non_snake_case)]
pub fn op_8xyE(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;

    // Save the Most Significant Bit in VF.
    cpu.v[0xF] = (cpu.v[vx as usize] & 0x80) >> 7;

    cpu.v[vx as usize] <<= 1;
}

/// Skip next instruction if Vx != Vy.
pub fn op_9xy0(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    if cpu.v[vx as usize] != cpu.v[vy as usize] {
        cpu.pc += 2;
    }
}

/// Set Index Register = nnn.
#[allow(non_snake_case)]
pub fn op_Annn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;

    cpu.i = address;
}

/// Jump to location nnn + V0.
#[allow(non_snake_case)]
pub fn op_Bnnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;

    cpu.pc = cpu.v[0] as u16 + address;
}

/// Set Vx = random byte AND kk
#[allow(non_snake_case)]
pub fn op_Cxkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let mut rng = rand::rng();
    let rand_byte: u8 = rng.random();

    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    cpu.v[vx as usize] = rand_byte & byte;
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy),
/// set VF = collision.
#[allow(non_snake_case)]
pub fn op_Dxyn(opcode: u16, cpu: &mut cpu::Cpu, display: &mut display::Display) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let vy = ((opcode & 0x00F0) >> 4) as usize;
    let height = (opcode & 0x000F) as usize;

    // Wrap if going beyond screen boundaries.
    let x_pos = cpu.v[vx] as usize % WIDTH;
    let y_pos = cpu.v[vy] as usize % HEIGHT;

    cpu.v[0xF] = 0;

    for row in 0..height {
        let y = y_pos + row;
        if y >= HEIGHT {
            break;
        }
        let sprite_byte = cpu.ram.data[cpu.i as usize + row];

        for col in 0..8_usize {
            let x = x_pos + col;
            if x >= WIDTH {
                break;
            }

            if sprite_byte & (0x80 >> col) != 0 {
                let screen_pixel = &mut display.video[y * WIDTH + x];
                if *screen_pixel == 0xFFFFFFFF {
                    cpu.v[0xF] = 1;
                }

                *screen_pixel ^= 0xFFFFFFFF;
            }
        }
    }
}
