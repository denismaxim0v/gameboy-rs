pub struct Bootrom {
    pub is_active: bool,
    pub data: Vec<u8>,
}

impl Bootrom {
    pub fn new(is_active: bool, rom_buf: Vec<u8>) -> Self {
        Self {
            is_active: is_active,
            data: rom_buf,
        }
    }
}
