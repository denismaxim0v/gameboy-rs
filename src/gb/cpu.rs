use super::registers::Registers;
use super::mmu::MMU;
pub struct CPU {
  registers: Registers,
  pc: u16,
  sp: u16
}

impl CPU {
  pub fn new() -> Self {
      Self {
          registers: Registers::new(),
          pc: 0x0,
          sp: 0x00
      }
  }

  pub fn fetch(&mut self, mmu: &MMU) -> u8 {
    let i = mmu.read(self.pc);
    self.pc = self.pc.wrapping_add(1);
    i
  }
}
