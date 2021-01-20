use super::registers::Registers;
use super::mmu::MMU;
pub struct CPU {
  registers: Registers,
}

impl CPU {
  pub fn new() -> Self {
      Self {
          registers: Registers::new(),
      }
  }

  pub fn fetch(&mut self, mmu: &MMU) -> u8 {
    let i = mmu.read(self.registers.pc);
    self.registers.pc = self.registers.pc.wrapping_add(1);
    i
  }
}
