use super::cpu::CPU;
use super::mmu::MMU;

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
  pub fn emulate_cycle(&self) {
      if self.mmu.bootrom.is_active {
          for &byte in self.mmu.bootrom.data.iter() {
              println!("{:#X}", byte as u16)
          }
      }
  }
}
