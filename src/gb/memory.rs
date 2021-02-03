pub struct Memory {
  pub data: Vec<u8>
}

impl Memory {
  pub fn new() -> Self {
      Self {
          data: Vec::new()
      }
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.data[addr as usize]
  }

  pub fn write(&mut self, addr: u16, v: u8) {
    self.data[addr as usize] = v
  }
}
