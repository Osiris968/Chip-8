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
