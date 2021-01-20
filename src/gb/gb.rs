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
  pub fn emulate_cycle(&mut self) {
      let byte = self.cpu.fetch(&self.mmu);
      match byte{
          0x31 => println!("{:?}", "found 0x3100"),
          _ => panic!("unknown opcode {:#X}", byte)
      }
  }
}
