use super::bootrom::Bootrom;
use super::memory::Memory;
use std::fs::File;
use std::io::Read;

pub struct MMU {
    pub bootrom: Bootrom,
    pub memory: Memory,
}

impl MMU {
    pub fn new(bootrom: bool) -> Self {
        let mut f = File::open("./src/roms/dmg_boot.bin").unwrap();
        let mut rom = Vec::new();
        f.read_to_end(&mut rom);
        Self {
            bootrom: Bootrom::new(bootrom, rom),
            memory: Memory::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => {
                if self.bootrom.is_active & (addr < 0x1000) {
                    self.bootrom.data[addr as usize]
                } else {
                    self.memory.data[addr as usize]
                }
            }
            _ => 0,
        }
    }
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => {
                if self.bootrom.is_active & (addr < 0x1000) {
                    self.bootrom.data[addr as usize] = value;
                } else {
                    self.memory.data[addr as usize] = value;
                }
            }
            _ => {}
        }
    }
    pub fn get_mut_mem(&mut self) -> &mut Memory {
        &mut self.memory
    }
}
