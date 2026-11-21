#![allow(unused)]

use crate::cpu;

/// Clear the display.
#[allow(non_snake_case)]
pub fn op_00E0(buffer: &mut [u32]) {
    buffer.fill(0);
}

/// Return from a subroutine.
#[allow(non_snake_case)]
pub fn op_00EE(cpu: &mut cpu::Cpu) {
    cpu.sp -= 1;
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

    cpu.v[vx as usize] += byte;
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

    let sum: u16 = (cpu.v[vx as usize] + cpu.v[vy as usize]) as u16;

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

    cpu.v[vx as usize] -= cpu.v[vy as usize];
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

    cpu.v[vx as usize] = cpu.v[vy as usize] - cpu.v[vx as usize];
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
