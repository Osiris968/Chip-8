#![allow(unused)]

use crate::cpu;

// Instruction 00E0
pub fn op_clear_screen(buffer: &mut [u32]) {
    buffer.fill(0);
}

// Instruction 00EE
pub fn op_return(cpu: &mut cpu::Cpu) {
    cpu.sp -= 1;
}

// Instruction 1nnn
pub fn op_jump(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = (opcode & 0x0FFF);
    cpu.pc = address;
}

pub fn op_call(opcode: u16, cpu: &mut cpu::Cpu) {
    let address = (opcode & 0x0FFF);

    cpu.stack[cpu.sp] = cpu.pc;
    cpu.sp += 1;

    cpu.pc = address;
}
