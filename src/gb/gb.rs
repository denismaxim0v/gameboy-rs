use super::cpu::CPU;
use super::mmu::MMU;
use super::linker::{link, link_prefix};
use super::opcodes::{Opcode, Operand, Condition};
use super::registers::{R8, R16};

pub struct GB {
  cpu: CPU,
  mmu: MMU
}

impl GB {
  pub fn new() -> Self {
      Self {
          cpu: CPU::new(),
          mmu: MMU::new(true)
      }
  }
  pub fn emulate_cycle(&mut self) -> u32 {
      let byte = self.cpu.fetch(&self.mmu);

      let opcode = match link(byte) {
        Some(Opcode::PREFIX) => link_prefix(byte),
        Some(opcode) => opcode,
        None => panic!("Unknown opcode {:#X}", byte)
      };

      let cycles = match opcode {
        Opcode::ADC(Operand::Reg8(R8::A), Operand::Reg8(r)) => {
          let v = self.cpu.registers.get_r8(r);
          println!("found opcode");
          1
        },
        opcode => panic!("uniplemented")
      };

      cycles * 4
  }
}
