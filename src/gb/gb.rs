use super::cpu::CPU;
use super::linker::{link, link_prefix};
use super::memory::Memory;
use super::mmu::MMU;
use super::opcodes::{Condition, Opcode, Operand};

pub struct GB {
    cpu: CPU,
    mmu: MMU,
}

impl GB {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(true),
        }
    }
    // immediates
    fn imm_u8(&mut self, mem: Memory) -> u8 {
        let v = mem.read(self.cpu.registers.pc);
        self.cpu.registers.pc = self.cpu.registers.pc.wrapping_add(1);

        v
    }

    fn imm_i8(&mut self, mem: Memory) -> i8 {
        let v = mem.read(self.cpu.registers.pc);
        self.cpu.registers.pc = self.cpu.registers.pc.wrapping_add(1);

        v as i8
    }

    fn imm_u16(&mut self, mem: Memory) -> u16 {
        let v = mem.read_word(self.cpu.registers.pc);
        self.cpu.registers.pc = self.cpu.registers.pc.wrapping_add(2);

        v
    }

    pub fn emulate_cycle(&mut self) -> u32 {
        use super::registers::R16::*;
        use super::registers::R8::*;
        use Opcode::*;
        use Operand::*;

        let byte = self.cpu.fetch(&self.mmu);

        let opcode = match link(byte) {
            Some(PREFIX) => link_prefix(byte),
            Some(opcode) => opcode,
            None => panic!("Unknown opcode {:#X}", byte),
        };

        let cycles = match opcode {
            ADC(Reg8(A), Reg8(r)) => {
                let v = self.cpu.registers.get_r8(r);
                println!("found opcode ADC");
                1
            }
            ADC(Reg8(A), Mem(HL)) => {
                let a = self.cpu.registers.get_r16(HL);
                let v = self.mmu.read(a);
                println!("found opcode ADC");
                2
            }
            opcode => panic!("uniplemented"),
        };

        cycles * 4
    }

    fn check_cond(&self, cond: Condition) -> bool {
        match cond {
            Condition::Always => true,
            Condition::Z => self.cpu.registers.flag.zero,
            Condition::NZ => !self.cpu.registers.flag.zero,
            Condition::C => self.cpu.registers.flag.carry,
            Condition::NC => !self.cpu.registers.flag.carry,
        }
    }
}
