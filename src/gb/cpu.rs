use super::mmu::MMU;
use super::registers::Registers;
use super::memory::Memory;
use super::linker::{link, link_prefix};
use super::opcodes::{Condition, Opcode, Operand};
use super::alu::cp;
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        }
    }

    pub fn fetch(&mut self, mmu: &MMU) -> u8 {
        let i = mmu.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        i
    }

    pub fn emulate_cycle(&mut self, mmu: &mut MMU) -> u32 {
        use super::registers::R16::*;
        use super::registers::R8::*;
        use Opcode::*;
        use Operand::*;

        let byte = self.fetch(mmu);

        let opcode = match link(byte) {
            Some(PREFIX) => link_prefix(byte),
            Some(opcode) => opcode,
            None => panic!("Unknown opcode {:#X}", byte),
        };

        let cycles = match opcode {
            ADC(Reg8(A), Reg8(r)) => {
                let v = self.registers.get_r8(r);
                println!("found opcode ADC");
                1
            }
            ADC(Reg8(A), Mem(HL)) => {
                let a = self.registers.get_r16(HL);
                let v = mmu.read(a);
                println!("found opcode ADC");
                2
            },
            LD(Reg16(dr), ImmU16) => {
                let v = self.imm_u16(mmu);
                self.registers.set_r16(dr, v);
                3
            },
            CP(Reg8(A), ImmU8) => {
                let v = self.imm_u8(mmu);
                cp(self, v);
                2
            }
            opcode => {
                println!("opcode {:?}", opcode);
                0
            },
        };

        cycles * 4
    }
    // immediates
    fn imm_u8(&mut self, mem: &MMU) -> u8 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        v
    }

    fn imm_i8(&mut self, mem: Memory) -> i8 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        v as i8
    }

    fn imm_u16(&mut self, mem: &MMU) -> u16 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        v as u16
    }

    fn check_cond(&self, cond: Condition) -> bool {
        match cond {
            Condition::Always => true,
            Condition::Z => self.registers.flag.zero,
            Condition::NZ => !self.registers.flag.zero,
            Condition::C => self.registers.flag.carry,
            Condition::NC => !self.registers.flag.carry,
        }
    }
}
