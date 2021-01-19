pub struct Registers {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub f: u8,
  pub h: u8,
  pub l: u8,
  pub flag: FlagRegister
}

impl Registers {
  pub fn new() -> Self {
      Self {
          a: 0,
          b: 0,
          c: 0,
          d: 0,
          e: 0,
          f: 0,
          h: 0,
          l: 0,
          flag: FlagRegister::new()
      }
  }
}

pub struct FlagRegister {
  pub zero: bool,
  pub subtract: bool,
  pub half_carry: bool,
  pub carry: bool,
}

impl FlagRegister {
  pub fn new() -> Self {
      Self {
          zero: false,
          subtract: false,
          half_carry: false,
          carry: false
      }
  }
}
