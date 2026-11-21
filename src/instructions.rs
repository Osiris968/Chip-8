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
