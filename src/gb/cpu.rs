use super::registers::Registers;

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
}
