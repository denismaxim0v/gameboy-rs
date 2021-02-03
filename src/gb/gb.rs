use super::cpu::CPU;
use super::linker::{link, link_prefix};
use super::mmu::MMU;
use super::opcodes::{Condition, Opcode, Operand};
use super::registers::{R16, R8};
use super::memory::Memory;

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
    pub fn emulate_cycle(&mut self) -> u32 {
        let byte = self.cpu.fetch(&self.mmu);

        let opcode = match link(byte) {
            Some(Opcode::PREFIX) => link_prefix(byte),
            Some(opcode) => opcode,
            None => panic!("Unknown opcode {:#X}", byte),
        };

        let cycles = match opcode {
            Opcode::ADC(Operand::Reg8(R8::A), Operand::Reg8(r)) => {
                let v = self.cpu.registers.get_r8(r);
                println!("found opcode");
                1
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
