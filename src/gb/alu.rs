use super::cpu::CPU;
use super::registers::*;

pub fn cp(cpu: &mut CPU, v: u8) {
    let a = cpu.registers.a;
    sub(cpu, v, false);
    cpu.registers.a = a;
}

pub fn sub(cpu: &mut CPU, b: u8, carry: bool) {
    let c = if carry && cpu.registers.flag.carry {
        1
    } else {
        0
    };
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(c);
    cpu.registers.flag.zero = r == 0;
    cpu.registers.flag.half_carry = (a & 0x0F) < (b & 0x0F) + c;
    cpu.registers.flag.negative = true;
    cpu.registers.flag.carry = (a as u16) < (b as u16) + (c as u16);
    cpu.registers.a = r;
}

pub fn xor(cpu: &mut CPU, b: u8) {
    let a = cpu.registers.a ^ b;
    cpu.registers.flag.zero = a == 0;
    cpu.registers.flag.negative = false;
    cpu.registers.flag.half_carry = false;
    cpu.registers.flag.carry = false;
    cpu.registers.a = a;
}

pub fn set(_cpu: &mut CPU, a: u8, b: u8) -> u8 {
  a | (1 << (b as u32))
}