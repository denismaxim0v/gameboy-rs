use std::fs::File;
use std::io::Read;
use std::env;

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

pub struct MMU {
    pub bootrom: Bootrom,
    pub memory: Memory
}

pub struct Bootrom {
    pub is_active: bool,
    pub data: Vec<u8>
}

impl Bootrom {
    pub fn new(is_active: bool, rom_buf: Vec<u8>) -> Self {
        Self {
            is_active: is_active,
            data: rom_buf
        }
    }
}


impl MMU {
    pub fn new(bootrom: bool) -> Self {
        let mut f = File::open("./src/dmg_boot.bin").unwrap();
        let mut rom = Vec::new();
        f.read_to_end(&mut rom);
        Self {
            bootrom: Bootrom::new(bootrom, rom),
            memory: Memory::new()
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
            _ => 0
        }
    }
}

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

fn main() {
    let args: Vec<String> = env::args().collect();

    let gb = GB::new();
    gb.emulate_cycle();
}
