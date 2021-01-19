pub struct Memory {
  pub data: Vec<u8>
}

impl Memory {
  pub fn new() -> Self {
      Self {
          data: Vec::new()
      }
  }
}
