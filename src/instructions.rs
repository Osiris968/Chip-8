#![allow(non_snake_case)]

use crate::{
    cpu,
    display::{self, HEIGHT, WIDTH},
};
use rand::RngExt;

/// Clear the display.
pub fn op_00E0(buffer: &mut [u32]) {
    buffer.fill(0);
}

/// Return from a subroutine.
pub fn op_00EE(cpu: &mut cpu::Cpu) {
    if cpu.sp == 0 {
        eprintln!(
            "Stack underflow: return with an empty stack at {:#05X}",
            cpu.pc - 2
        );
        return;
    }

    cpu.sp -= 1;

    cpu.pc = cpu.stack[cpu.sp];
}

/// Jump to location nnn.
pub fn op_1nnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;
    cpu.pc = address;
}

/// Call subroutine at nnn.
pub fn op_2nnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;

    if cpu.sp >= cpu.stack.len() {
        eprintln!("Stack overflow: call to {:#05X} with a full stack", address);
        return;
    }

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

    let (result, carry) = cpu.v[vx as usize].overflowing_add(cpu.v[vy as usize]);

    // Write VF last so it isn't clobbered when x is F.
    cpu.v[vx as usize] = result;
    cpu.v[0xF] = carry as u8;
}

/// Set Vx = Vx - Vy, set VF = NOT borrow.
/// If Vx > Vy, then VF is set to 1, otherwise 0.
/// Then Vy is subtracted from Vx, and the results stored in Vx.
pub fn op_8xy5(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    let (result, borrow) = cpu.v[vx as usize].overflowing_sub(cpu.v[vy as usize]);

    // Write VF last so it isn't clobbered when x is F.
    cpu.v[vx as usize] = result;
    cpu.v[0xF] = !borrow as u8;
}

/// Set Vx = Vx SHR 1.
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
/// Then Vx is divided by 2.
pub fn op_8xy6(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;

    // Save the Least Significant Bit before shifting.
    let lsb = cpu.v[vx as usize] & 0x1;

    // Write VF last so it isn't clobbered when x is F.
    cpu.v[vx as usize] >>= 1;
    cpu.v[0xF] = lsb;
}

/// Set Vx = Vy - Vx, set VF = NOT borrow.
/// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy,
/// and the results stored in Vx.
pub fn op_8xy7(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;
    let vy = (opcode & 0x00F0) >> 4;

    let (result, borrow) = cpu.v[vy as usize].overflowing_sub(cpu.v[vx as usize]);

    // Write VF last so it isn't clobbered when x is F.
    cpu.v[vx as usize] = result;
    cpu.v[0xF] = !borrow as u8;
}

/// Set Vx = Vx SHL 1
/// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
/// Then Vx is multiplied by 2.
pub fn op_8xyE(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = (opcode & 0x0F00) >> 8;

    // Save the Most Significant Bit before shifting.
    let msb = (cpu.v[vx as usize] & 0x80) >> 7;

    // Write VF last so it isn't clobbered when x is F.
    cpu.v[vx as usize] <<= 1;
    cpu.v[0xF] = msb;
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
pub fn op_Annn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;

    cpu.i = address;
}

/// Jump to location nnn + V0.
pub fn op_Bnnn(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = opcode & 0x0FFF;

    cpu.pc = cpu.v[0] as u16 + address;
}

/// Set Vx = random byte AND kk
pub fn op_Cxkk(opcode: u16, cpu: &mut cpu::Cpu) {
    let mut rng = rand::rng();
    let rand_byte: u8 = rng.random();

    let vx = (opcode & 0x0F00) >> 8;
    let byte = (opcode & 0x00FF) as u8;

    cpu.v[vx as usize] = rand_byte & byte;
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy),
/// set VF = collision.
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
        // Stop if the sprite runs past the end of RAM.
        let Some(&sprite_byte) = cpu.ram.data.get(cpu.i as usize + row) else {
            break;
        };

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

/// Skip next instruction if key with the value of Vx is pressed.
pub fn op_Ex9E(opcode: u16, cpu: &mut cpu::Cpu, keypad: &[bool]) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let key = (cpu.v[vx] & 0xF) as usize;

    if keypad[key] {
        cpu.pc += 2;
    }
}

/// Skip next instruction if key with the value of Vx is not pressed.
pub fn op_ExA1(opcode: u16, cpu: &mut cpu::Cpu, keypad: &[bool]) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let key = (cpu.v[vx] & 0xF) as usize;

    if !keypad[key] {
        cpu.pc += 2;
    }
}

/// Set Vx = delay timer value.
pub fn op_Fx07(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    cpu.v[vx] = cpu.delay_timer;
}

/// Wait for a key press, store the value of the key in Vx.
pub fn op_Fx0A(opcode: u16, cpu: &mut cpu::Cpu, keypad: &[bool]) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    match keypad.iter().position(|&pressed| pressed) {
        Some(key) => cpu.v[vx] = key as u8,
        None => cpu.pc -= 2, // No key yet: run this instruction again.
    }
}

/// Set delay timer = Vx.
pub fn op_Fx15(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    cpu.delay_timer = cpu.v[vx];
}

/// Set sound timer = Vx.
pub fn op_Fx18(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    cpu.sound_timer = cpu.v[vx];
}

/// Set I = I + Vx.
pub fn op_Fx1E(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    cpu.i = cpu.i.wrapping_add(cpu.v[vx] as u16);
}

/// Set I = location of sprite for digit Vx.
pub fn op_Fx29(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let digit = (cpu.v[vx] & 0xF) as u16;

    cpu.i = 0x50 + (5 * digit);
}

/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
pub fn op_Fx33(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;
    let value = cpu.v[vx];
    let start = cpu.i as usize;

    // Skip the instruction if I..I+2 runs past the end of RAM.
    let Some(digits) = cpu.ram.data.get_mut(start..start + 3) else {
        eprintln!("Fx33: I = {:#05X} is out of bounds", cpu.i);
        return;
    };

    digits[0] = value / 100; // Hundreds place
    digits[1] = (value / 10) % 10; // Tens place
    digits[2] = value % 10; // Ones place
}

/// Store registers V0 through Vx in memory starting at location I.
pub fn op_Fx55(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    let start = cpu.i as usize;

    // Skip the instruction if I..=I+x runs past the end of RAM.
    let Some(dest) = cpu.ram.data.get_mut(start..=start + vx) else {
        eprintln!("Fx55: I = {:#05X} is out of bounds", cpu.i);
        return;
    };

    dest.copy_from_slice(&cpu.v[..=vx]);
}

/// Read registers V0 through Vx from memory starting at location I.
pub fn op_Fx65(opcode: u16, cpu: &mut cpu::Cpu) {
    let vx = ((opcode & 0x0F00) >> 8) as usize;

    let start = cpu.i as usize;

    // Skip the instruction if I..=I+x runs past the end of RAM.
    let Some(src) = cpu.ram.data.get(start..=start + vx) else {
        eprintln!("Fx65: I = {:#05X} is out of bounds", cpu.i);
        return;
    };

    cpu.v[..=vx].copy_from_slice(src);
}
