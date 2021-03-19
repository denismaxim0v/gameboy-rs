use super::cpu::CPU;
use super::memory::Memory;
use super::mmu::MMU;

pub struct GB {
    cpu: CPU,
    pub mmu: MMU,
}

impl GB {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(true),
        }
    }
    pub fn step(&mut self) -> u32 {
        self.cpu.emulate_cycle(&mut self.mmu)
    }
}
