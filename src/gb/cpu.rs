use super::alu::{cp, set, xor};
use super::linker::{link, link_prefix};
use super::memory::Memory;
use super::mmu::MMU;
use super::opcodes::{Condition, Opcode, Operand};
use super::registers::Registers;
pub struct CPU {
    pub registers: Registers,
    halted: bool,
    ime: bool,
    ime_next: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            halted: false,
            ime: false,
            ime_next: false
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
            None => NOP
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
            }
            LD(Reg16(dr), ImmU16) => {
                let v = self.imm_u16(mmu);
                self.registers.set_r16(dr, v);
                3
            }
            LD(Reg8(A), Reg8(H)) => {
                let v = self.registers.get_r8(H);
                self.registers.set_r8(A, v);
                1
            },
            CP(Reg8(A), ImmU8) => {
                let v = self.imm_u8(mmu);
                cp(self, v);
                2
            }
            XOR(Reg8(A), Reg8(A)) => {
                let v = self.registers.get_r8(A);
                xor(self, v);
                1
            }
            LDD(Mem(HL), Reg8(A)) => {
                let addr = self.registers.get_r16(HL);
                mmu.write(addr, self.registers.a);
                self.registers.set_r16(HL, addr.wrapping_sub(1));
                2
            }
            SET(b, Reg8(E)) => {
                let a = self.registers.get_r8(E);
                let v = set(self, mmu.read(a as u16), b);
                mmu.write(a as u16, v);
                2
            }
            JR(cond, ImmI8) => {
                let o = self.imm_i8(mmu);
                let a = self.add_u16_i8(self.registers.pc, o);
                if self.check_cond(cond) {
                    self.registers.pc = a;
                    3
                } else {
                    2
                }
            }
            DI => {
                self.ime_next = false;
                1
            }
            EI => {
                self.ime_next = true;
                1
            }
            opcode => {
                println!("opcode {:?}", opcode);
                0
            }
        };

        cycles * 4
    }
    fn cpu_enable_interrupt(&mut self) {
        self.ime = true;
        // TODO clock
    }
    fn cpu_flag_set_zero(&mut self, value: bool) {
        self.registers.flag.zero = value;
    }
    fn cpu_flag_set_neg(&mut self, value: bool) {
        self.registers.flag.negative = value;
    }
    fn cpu_flag_self_hc(&mut self, value: bool) {
        self.registers.flag.half_carry = value;
    }
    fn cpu_flag_set_carry(&mut self, value: bool) {
        self.registers.flag.carry = value;
    }
    // immediates
    fn imm_u8(&mut self, mem: &MMU) -> u8 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        v
    }

    fn imm_i8(&mut self, mem: &MMU) -> i8 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        v as i8
    }

    fn imm_u16(&mut self, mem: &MMU) -> u16 {
        let v = mem.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(2);

        v as u16
    }

    fn add_u16_i8(&self, a: u16, b: i8) -> u16 {
        ((a as u32 as i32) + b as i32) as u16
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
