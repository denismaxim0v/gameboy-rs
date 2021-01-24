use super::opcodes::*;
use super::registers::R16::*;
use super::registers::R8::*;


pub fn link(opcode: u8) -> Option<Opcode> {
  use Opcode::*;
  use Operand::*;

  match opcode {
    0x00 => Some(NOP),
        0x01 => Some(LD(Reg16(BC), ImmU16)),
        0x02 => Some(LD(Mem(BC), Reg8(A))),
        0x03 => Some(INC(Reg16(BC))),
        0x04 => Some(INC(Reg8(B))),
        0x05 => Some(DEC(Reg8(B))),
        0x06 => Some(LD(Reg8(B), ImmU8)),
        0x07 => Some(RLCA),
        0x08 => Some(LD(MemImmU16, Reg16(SP))),
        0x09 => Some(ADD(Reg16(HL), Reg16(BC))),
        0x0A => Some(LD(Reg8(A), Mem(BC))),
        0x0B => Some(DEC(Reg16(BC))),
        0x0C => Some(INC(Reg8(C))),
        0x0D => Some(DEC(Reg8(C))),
        0x0E => Some(LD(Reg8(C), ImmU8)),
        0x0F => Some(RRCA),
        0x10 => Some(STOP),
        0x11 => Some(LD(Reg16(DE), ImmU16)),
        0x12 => Some(LD(Mem(DE), Reg8(A))),
        0x13 => Some(INC(Reg16(DE))),
        0x14 => Some(INC(Reg8(D))),
        0x15 => Some(DEC(Reg8(D))),
        0x16 => Some(LD(Reg8(D), ImmU8)),
        0x17 => Some(RLA),
        0x18 => Some(JR(Cond::Always, ImmI8)),
        0x19 => Some(ADD(Reg16(HL), Reg16(DE))),
        0x1A => Some(LD(Reg8(A), Mem(DE))),
        0x1B => Some(DEC(Reg16(DE))),
        0x1C => Some(INC(Reg8(E))),
        0x1D => Some(DEC(Reg8(E))),
        0x1E => Some(LD(Reg8(E), ImmU8)),
        0x1F => Some(RRA),
        0x20 => Some(JR(Cond::NZ, ImmI8)),
        0x21 => Some(LD(Reg16(HL), ImmU16)),
        0x22 => Some(LDI(Mem(HL), Reg8(A))),
        0x23 => Some(INC(Reg16(HL))),
        0x24 => Some(INC(Reg8(H))),
        0x25 => Some(DEC(Reg8(H))),
        0x26 => Some(LD(Reg8(H), ImmU8)),
        0x27 => Some(DAA),
        0x28 => Some(JR(Cond::Z, ImmI8)),
        0x29 => Some(ADD(Reg16(HL), Reg16(HL))),
        0x2A => Some(LDI(Reg8(A), Mem(HL))),
        0x2B => Some(DEC(Reg16(HL))),
        0x2C => Some(INC(Reg8(L))),
        0x2D => Some(DEC(Reg8(L))),
        0x2E => Some(LD(Reg8(L), ImmU8)),
        0x2F => Some(CPL),
        0x30 => Some(JR(Cond::NC, ImmI8)),
        0x31 => Some(LD(Reg16(SP), ImmU16)),
        0x32 => Some(LDD(Mem(HL), Reg8(A))),
        0x33 => Some(INC(Reg16(SP))),
        0x34 => Some(INC(Mem(HL))),
        0x35 => Some(DEC(Mem(HL))),
        0x36 => Some(LD(Mem(HL), ImmU8)),
        0x37 => Some(SCF),
        0x38 => Some(JR(Cond::C, ImmI8)),
        0x39 => Some(ADD(Reg16(HL), Reg16(SP))),
        0x3A => Some(LDD(Reg8(A), Mem(HL))),
        0x3B => Some(DEC(Reg16(SP))),
        0x3C => Some(INC(Reg8(A))),
        0x3D => Some(DEC(Reg8(A))),
        0x3E => Some(LD(Reg8(A), ImmU8)),
        0x3F => Some(CCF),
        0x40 => Some(LD(Reg8(B), Reg8(B))),
        0x41 => Some(LD(Reg8(B), Reg8(C))),
        0x42 => Some(LD(Reg8(B), Reg8(D))),
        0x43 => Some(LD(Reg8(B), Reg8(E))),
        0x44 => Some(LD(Reg8(B), Reg8(H))),
        0x45 => Some(LD(Reg8(B), Reg8(L))),
        0x46 => Some(LD(Reg8(B), Mem(HL))),
        0x47 => Some(LD(Reg8(B), Reg8(A))),
        0x48 => Some(LD(Reg8(C), Reg8(B))),
        0x49 => Some(LD(Reg8(C), Reg8(C))),
        0x4A => Some(LD(Reg8(C), Reg8(D))),
        0x4B => Some(LD(Reg8(C), Reg8(E))),
        0x4C => Some(LD(Reg8(C), Reg8(H))),
        0x4D => Some(LD(Reg8(C), Reg8(L))),
        0x4E => Some(LD(Reg8(C), Mem(HL))),
        0x4F => Some(LD(Reg8(C), Reg8(A))),
        0x50 => Some(LD(Reg8(D), Reg8(B))),
        0x51 => Some(LD(Reg8(D), Reg8(C))),
        0x52 => Some(LD(Reg8(D), Reg8(D))),
        0x53 => Some(LD(Reg8(D), Reg8(E))),
        0x54 => Some(LD(Reg8(D), Reg8(H))),
        0x55 => Some(LD(Reg8(D), Reg8(L))),
        0x56 => Some(LD(Reg8(D), Mem(HL))),
        0x57 => Some(LD(Reg8(D), Reg8(A))),
        0x58 => Some(LD(Reg8(E), Reg8(B))),
        0x59 => Some(LD(Reg8(E), Reg8(C))),
        0x5A => Some(LD(Reg8(E), Reg8(D))),
        0x5B => Some(LD(Reg8(E), Reg8(E))),
        0x5C => Some(LD(Reg8(E), Reg8(H))),
        0x5D => Some(LD(Reg8(E), Reg8(L))),
        0x5E => Some(LD(Reg8(E), Mem(HL))),
        0x5F => Some(LD(Reg8(E), Reg8(A))),
        0x60 => Some(LD(Reg8(H), Reg8(B))),
        0x61 => Some(LD(Reg8(H), Reg8(C))),
        0x62 => Some(LD(Reg8(H), Reg8(D))),
        0x63 => Some(LD(Reg8(H), Reg8(E))),
        0x64 => Some(LD(Reg8(H), Reg8(H))),
        0x65 => Some(LD(Reg8(H), Reg8(L))),
        0x66 => Some(LD(Reg8(H), Mem(HL))),
        0x67 => Some(LD(Reg8(H), Reg8(A))),
        0x68 => Some(LD(Reg8(L), Reg8(B))),
        0x69 => Some(LD(Reg8(L), Reg8(C))),
        0x6A => Some(LD(Reg8(L), Reg8(D))),
        0x6B => Some(LD(Reg8(L), Reg8(E))),
        0x6C => Some(LD(Reg8(L), Reg8(H))),
        0x6D => Some(LD(Reg8(L), Reg8(L))),
        0x6E => Some(LD(Reg8(L), Mem(HL))),
        0x6F => Some(LD(Reg8(L), Reg8(A))),
        0x70 => Some(LD(Mem(HL), Reg8(B))),
        0x71 => Some(LD(Mem(HL), Reg8(C))),
        0x72 => Some(LD(Mem(HL), Reg8(D))),
        0x73 => Some(LD(Mem(HL), Reg8(E))),
        0x74 => Some(LD(Mem(HL), Reg8(H))),
        0x75 => Some(LD(Mem(HL), Reg8(L))),
        0x76 => Some(HALT),
        0x77 => Some(LD(Mem(HL), Reg8(A))),
        0x78 => Some(LD(Reg8(A), Reg8(B))),
        0x79 => Some(LD(Reg8(A), Reg8(C))),
        0x7A => Some(LD(Reg8(A), Reg8(D))),
        0x7B => Some(LD(Reg8(A), Reg8(E))),
        0x7C => Some(LD(Reg8(A), Reg8(H))),
        0x7D => Some(LD(Reg8(A), Reg8(L))),
        0x7E => Some(LD(Reg8(A), Mem(HL))),
        0x7F => Some(LD(Reg8(A), Reg8(A))),
        0x80 => Some(ADD(Reg8(A), Reg8(B))),
        0x81 => Some(ADD(Reg8(A), Reg8(C))),
        0x82 => Some(ADD(Reg8(A), Reg8(D))),
        0x83 => Some(ADD(Reg8(A), Reg8(E))),
        0x84 => Some(ADD(Reg8(A), Reg8(H))),
        0x85 => Some(ADD(Reg8(A), Reg8(L))),
        0x86 => Some(ADD(Reg8(A), Mem(HL))),
        0x87 => Some(ADD(Reg8(A), Reg8(A))),
        0x88 => Some(ADC(Reg8(A), Reg8(B))),
        0x89 => Some(ADC(Reg8(A), Reg8(C))),
        0x8A => Some(ADC(Reg8(A), Reg8(D))),
        0x8B => Some(ADC(Reg8(A), Reg8(E))),
        0x8C => Some(ADC(Reg8(A), Reg8(H))),
        0x8D => Some(ADC(Reg8(A), Reg8(L))),
        0x8E => Some(ADC(Reg8(A), Mem(HL))),
        0x8F => Some(ADC(Reg8(A), Reg8(A))),
        0x90 => Some(SUB(Reg8(A), Reg8(B))),
        0x91 => Some(SUB(Reg8(A), Reg8(C))),
        0x92 => Some(SUB(Reg8(A), Reg8(D))),
        0x93 => Some(SUB(Reg8(A), Reg8(E))),
        0x94 => Some(SUB(Reg8(A), Reg8(H))),
        0x95 => Some(SUB(Reg8(A), Reg8(L))),
        0x96 => Some(SUB(Reg8(A), Mem(HL))),
        0x97 => Some(SUB(Reg8(A), Reg8(A))),
        0x98 => Some(SBC(Reg8(A), Reg8(B))),
        0x99 => Some(SBC(Reg8(A), Reg8(C))),
        0x9A => Some(SBC(Reg8(A), Reg8(D))),
        0x9B => Some(SBC(Reg8(A), Reg8(E))),
        0x9C => Some(SBC(Reg8(A), Reg8(H))),
        0x9D => Some(SBC(Reg8(A), Reg8(L))),
        0x9E => Some(SBC(Reg8(A), Mem(HL))),
        0x9F => Some(SBC(Reg8(A), Reg8(A))),
        0xA0 => Some(AND(Reg8(A), Reg8(B))),
        0xA1 => Some(AND(Reg8(A), Reg8(C))),
        0xA2 => Some(AND(Reg8(A), Reg8(D))),
        0xA3 => Some(AND(Reg8(A), Reg8(E))),
        0xA4 => Some(AND(Reg8(A), Reg8(H))),
        0xA5 => Some(AND(Reg8(A), Reg8(L))),
        0xA6 => Some(AND(Reg8(A), Mem(HL))),
        0xA7 => Some(AND(Reg8(A), Reg8(A))),
        0xA8 => Some(XOR(Reg8(A), Reg8(B))),
        0xA9 => Some(XOR(Reg8(A), Reg8(C))),
        0xAA => Some(XOR(Reg8(A), Reg8(D))),
        0xAB => Some(XOR(Reg8(A), Reg8(E))),
        0xAC => Some(XOR(Reg8(A), Reg8(H))),
        0xAD => Some(XOR(Reg8(A), Reg8(L))),
        0xAE => Some(XOR(Reg8(A), Mem(HL))),
        0xAF => Some(XOR(Reg8(A), Reg8(A))),
        0xB0 => Some(OR(Reg8(A), Reg8(B))),
        0xB1 => Some(OR(Reg8(A), Reg8(C))),
        0xB2 => Some(OR(Reg8(A), Reg8(D))),
        0xB3 => Some(OR(Reg8(A), Reg8(E))),
        0xB4 => Some(OR(Reg8(A), Reg8(H))),
        0xB5 => Some(OR(Reg8(A), Reg8(L))),
        0xB6 => Some(OR(Reg8(A), Mem(HL))),
        0xB7 => Some(OR(Reg8(A), Reg8(A))),
        0xB8 => Some(CP(Reg8(A), Reg8(B))),
        0xB9 => Some(CP(Reg8(A), Reg8(C))),
        0xBA => Some(CP(Reg8(A), Reg8(D))),
        0xBB => Some(CP(Reg8(A), Reg8(E))),
        0xBC => Some(CP(Reg8(A), Reg8(H))),
        0xBD => Some(CP(Reg8(A), Reg8(L))),
        0xBE => Some(CP(Reg8(A), Mem(HL))),
        0xBF => Some(CP(Reg8(A), Reg8(A))),
        0xC0 => Some(RET(Cond::NZ)),
        0xC1 => Some(POP(Reg16(BC))),
        0xC2 => Some(JP(Cond::NZ, ImmU16)),
        0xC3 => Some(JP(Cond::Always, ImmU16)),
        0xC4 => Some(CALL(Cond::NZ, ImmU16)),
        0xC5 => Some(PUSH(Reg16(BC))),
        0xC6 => Some(ADD(Reg8(A), ImmU8)),
        0xC7 => Some(RST(0x00)),
        0xC8 => Some(RET(Cond::Z)),
        0xC9 => Some(RET(Cond::Always)),
        0xCA => Some(JP(Cond::Z, ImmU16)),
        0xCB => Some(PREFIX),
        0xCC => Some(CALL(Cond::Z, ImmU16)),
        0xCD => Some(CALL(Cond::Always, ImmU16)),
        0xCE => Some(ADC(Reg8(A), ImmU8)),
        0xCF => Some(RST(0x08)),
        0xD0 => Some(RET(Cond::NC)),
        0xD1 => Some(POP(Reg16(DE))),
        0xD2 => Some(JP(Cond::NC, ImmU16)),
        0xD3 => None,
        0xD4 => Some(CALL(Cond::NC, ImmU16)),
        0xD5 => Some(PUSH(Reg16(DE))),
        0xD6 => Some(SUB(Reg8(A), ImmU8)),
        0xD7 => Some(RST(0x10)),
        0xD8 => Some(RET(Cond::C)),
        0xD9 => Some(RETI),
        0xDA => Some(JP(Cond::C, ImmU16)),
        0xDB => None,
        0xDC => Some(CALL(Cond::C, ImmU16)),
        0xDD => None,
        0xDE => Some(SBC(Reg8(A), ImmU8)),
        0xDF => Some(RST(0x18)),
        0xE0 => Some(LD(ZMemImmU8, Reg8(A))),
        0xE1 => Some(POP(Reg16(HL))),
        0xE2 => Some(LD(ZMem(C), Reg8(A))),
        0xE3 => None,
        0xE4 => None,
        0xE5 => Some(PUSH(Reg16(HL))),
        0xE6 => Some(AND(Reg8(A), ImmU8)),
        0xE7 => Some(RST(0x20)),
        0xE8 => Some(ADD(Reg16(SP), ImmI8)),
        0xE9 => Some(JP(Cond::Always, Reg16(HL))),
        0xEA => Some(LD(MemImmU16, Reg8(A))),
        0xEB => None,
        0xEC => None,
        0xED => None,
        0xEE => Some(XOR(Reg8(A), ImmU8)),
        0xEF => Some(RST(0x28)),
        0xF0 => Some(LD(Reg8(A), ZMemImmU8)),
        0xF1 => Some(POP(Reg16(AF))),
        0xF2 => Some(LD(Reg8(A), ZMem(C))),
        0xF3 => Some(DI),
        0xF4 => None,
        0xF5 => Some(PUSH(Reg16(AF))),
        0xF6 => Some(OR(Reg8(A), ImmU8)),
        0xF7 => Some(RST(0x30)),
        0xF8 => Some(LD(Reg16(HL), SPImmI8)),
        0xF9 => Some(LD(Reg16(SP), Reg16(HL))),
        0xFA => Some(LD(Reg8(A), MemImmU16)),
        0xFB => Some(EI),
        0xFC => None,
        0xFD => None,
        0xFE => Some(CP(Reg8(A), ImmU8)),
        0xFF => Some(RST(0x38)),
  }
}

pub fn decode_prefix(opcode: u8) -> Opcode {
  use Opcode::*;
  use Oper::*;

  match opcode {
      0x00 => RLC(Reg8(B)),
      0x01 => RLC(Reg8(C)),
      0x02 => RLC(Reg8(D)),
      0x03 => RLC(Reg8(E)),
      0x04 => RLC(Reg8(H)),
      0x05 => RLC(Reg8(L)),
      0x06 => RLC(Mem(HL)),
      0x07 => RLC(Reg8(A)),
      0x08 => RRC(Reg8(B)),
      0x09 => RRC(Reg8(C)),
      0x0A => RRC(Reg8(D)),
      0x0B => RRC(Reg8(E)),
      0x0C => RRC(Reg8(H)),
      0x0D => RRC(Reg8(L)),
      0x0E => RRC(Mem(HL)),
      0x0F => RRC(Reg8(A)),
      0x10 => RL(Reg8(B)),
      0x11 => RL(Reg8(C)),
      0x12 => RL(Reg8(D)),
      0x13 => RL(Reg8(E)),
      0x14 => RL(Reg8(H)),
      0x15 => RL(Reg8(L)),
      0x16 => RL(Mem(HL)),
      0x17 => RL(Reg8(A)),
      0x18 => RR(Reg8(B)),
      0x19 => RR(Reg8(C)),
      0x1A => RR(Reg8(D)),
      0x1B => RR(Reg8(E)),
      0x1C => RR(Reg8(H)),
      0x1D => RR(Reg8(L)),
      0x1E => RR(Mem(HL)),
      0x1F => RR(Reg8(A)),
      0x20 => SLA(Reg8(B)),
      0x21 => SLA(Reg8(C)),
      0x22 => SLA(Reg8(D)),
      0x23 => SLA(Reg8(E)),
      0x24 => SLA(Reg8(H)),
      0x25 => SLA(Reg8(L)),
      0x26 => SLA(Mem(HL)),
      0x27 => SLA(Reg8(A)),
      0x28 => SRA(Reg8(B)),
      0x29 => SRA(Reg8(C)),
      0x2A => SRA(Reg8(D)),
      0x2B => SRA(Reg8(E)),
      0x2C => SRA(Reg8(H)),
      0x2D => SRA(Reg8(L)),
      0x2E => SRA(Mem(HL)),
      0x2F => SRA(Reg8(A)),
      0x30 => SWAP(Reg8(B)),
      0x31 => SWAP(Reg8(C)),
      0x32 => SWAP(Reg8(D)),
      0x33 => SWAP(Reg8(E)),
      0x34 => SWAP(Reg8(H)),
      0x35 => SWAP(Reg8(L)),
      0x36 => SWAP(Mem(HL)),
      0x37 => SWAP(Reg8(A)),
      0x38 => SRL(Reg8(B)),
      0x39 => SRL(Reg8(C)),
      0x3A => SRL(Reg8(D)),
      0x3B => SRL(Reg8(E)),
      0x3C => SRL(Reg8(H)),
      0x3D => SRL(Reg8(L)),
      0x3E => SRL(Mem(HL)),
      0x3F => SRL(Reg8(A)),
      0x40 => BIT(0, Reg8(B)),
      0x41 => BIT(0, Reg8(C)),
      0x42 => BIT(0, Reg8(D)),
      0x43 => BIT(0, Reg8(E)),
      0x44 => BIT(0, Reg8(H)),
      0x45 => BIT(0, Reg8(L)),
      0x46 => BIT(0, Mem(HL)),
      0x47 => BIT(0, Reg8(A)),
      0x48 => BIT(1, Reg8(B)),
      0x49 => BIT(1, Reg8(C)),
      0x4A => BIT(1, Reg8(D)),
      0x4B => BIT(1, Reg8(E)),
      0x4C => BIT(1, Reg8(H)),
      0x4D => BIT(1, Reg8(L)),
      0x4E => BIT(1, Mem(HL)),
      0x4F => BIT(1, Reg8(A)),
      0x50 => BIT(2, Reg8(B)),
      0x51 => BIT(2, Reg8(C)),
      0x52 => BIT(2, Reg8(D)),
      0x53 => BIT(2, Reg8(E)),
      0x54 => BIT(2, Reg8(H)),
      0x55 => BIT(2, Reg8(L)),
      0x56 => BIT(2, Mem(HL)),
      0x57 => BIT(2, Reg8(A)),
      0x58 => BIT(3, Reg8(B)),
      0x59 => BIT(3, Reg8(C)),
      0x5A => BIT(3, Reg8(D)),
      0x5B => BIT(3, Reg8(E)),
      0x5C => BIT(3, Reg8(H)),
      0x5D => BIT(3, Reg8(L)),
      0x5E => BIT(3, Mem(HL)),
      0x5F => BIT(3, Reg8(A)),
      0x60 => BIT(4, Reg8(B)),
      0x61 => BIT(4, Reg8(C)),
      0x62 => BIT(4, Reg8(D)),
      0x63 => BIT(4, Reg8(E)),
      0x64 => BIT(4, Reg8(H)),
      0x65 => BIT(4, Reg8(L)),
      0x66 => BIT(4, Mem(HL)),
      0x67 => BIT(4, Reg8(A)),
      0x68 => BIT(5, Reg8(B)),
      0x69 => BIT(5, Reg8(C)),
      0x6A => BIT(5, Reg8(D)),
      0x6B => BIT(5, Reg8(E)),
      0x6C => BIT(5, Reg8(H)),
      0x6D => BIT(5, Reg8(L)),
      0x6E => BIT(5, Mem(HL)),
      0x6F => BIT(5, Reg8(A)),
      0x70 => BIT(6, Reg8(B)),
      0x71 => BIT(6, Reg8(C)),
      0x72 => BIT(6, Reg8(D)),
      0x73 => BIT(6, Reg8(E)),
      0x74 => BIT(6, Reg8(H)),
      0x75 => BIT(6, Reg8(L)),
      0x76 => BIT(6, Mem(HL)),
      0x77 => BIT(6, Reg8(A)),
      0x78 => BIT(7, Reg8(B)),
      0x79 => BIT(7, Reg8(C)),
      0x7A => BIT(7, Reg8(D)),
      0x7B => BIT(7, Reg8(E)),
      0x7C => BIT(7, Reg8(H)),
      0x7D => BIT(7, Reg8(L)),
      0x7E => BIT(7, Mem(HL)),
      0x7F => BIT(7, Reg8(A)),
      0x80 => RES(0, Reg8(B)),
      0x81 => RES(0, Reg8(C)),
      0x82 => RES(0, Reg8(D)),
      0x83 => RES(0, Reg8(E)),
      0x84 => RES(0, Reg8(H)),
      0x85 => RES(0, Reg8(L)),
      0x86 => RES(0, Mem(HL)),
      0x87 => RES(0, Reg8(A)),
      0x88 => RES(1, Reg8(B)),
      0x89 => RES(1, Reg8(C)),
      0x8A => RES(1, Reg8(D)),
      0x8B => RES(1, Reg8(E)),
      0x8C => RES(1, Reg8(H)),
      0x8D => RES(1, Reg8(L)),
      0x8E => RES(1, Mem(HL)),
      0x8F => RES(1, Reg8(A)),
      0x90 => RES(2, Reg8(B)),
      0x91 => RES(2, Reg8(C)),
      0x92 => RES(2, Reg8(D)),
      0x93 => RES(2, Reg8(E)),
      0x94 => RES(2, Reg8(H)),
      0x95 => RES(2, Reg8(L)),
      0x96 => RES(2, Mem(HL)),
      0x97 => RES(2, Reg8(A)),
      0x98 => RES(3, Reg8(B)),
      0x99 => RES(3, Reg8(C)),
      0x9A => RES(3, Reg8(D)),
      0x9B => RES(3, Reg8(E)),
      0x9C => RES(3, Reg8(H)),
      0x9D => RES(3, Reg8(L)),
      0x9E => RES(3, Mem(HL)),
      0x9F => RES(3, Reg8(A)),
      0xA0 => RES(4, Reg8(B)),
      0xA1 => RES(4, Reg8(C)),
      0xA2 => RES(4, Reg8(D)),
      0xA3 => RES(4, Reg8(E)),
      0xA4 => RES(4, Reg8(H)),
      0xA5 => RES(4, Reg8(L)),
      0xA6 => RES(4, Mem(HL)),
      0xA7 => RES(4, Reg8(A)),
      0xA8 => RES(5, Reg8(B)),
      0xA9 => RES(5, Reg8(C)),
      0xAA => RES(5, Reg8(D)),
      0xAB => RES(5, Reg8(E)),
      0xAC => RES(5, Reg8(H)),
      0xAD => RES(5, Reg8(L)),
      0xAE => RES(5, Mem(HL)),
      0xAF => RES(5, Reg8(A)),
      0xB0 => RES(6, Reg8(B)),
      0xB1 => RES(6, Reg8(C)),
      0xB2 => RES(6, Reg8(D)),
      0xB3 => RES(6, Reg8(E)),
      0xB4 => RES(6, Reg8(H)),
      0xB5 => RES(6, Reg8(L)),
      0xB6 => RES(6, Mem(HL)),
      0xB7 => RES(6, Reg8(A)),
      0xB8 => RES(7, Reg8(B)),
      0xB9 => RES(7, Reg8(C)),
      0xBA => RES(7, Reg8(D)),
      0xBB => RES(7, Reg8(E)),
      0xBC => RES(7, Reg8(H)),
      0xBD => RES(7, Reg8(L)),
      0xBE => RES(7, Mem(HL)),
      0xBF => RES(7, Reg8(A)),
      0xC0 => SET(0, Reg8(B)),
      0xC1 => SET(0, Reg8(C)),
      0xC2 => SET(0, Reg8(D)),
      0xC3 => SET(0, Reg8(E)),
      0xC4 => SET(0, Reg8(H)),
      0xC5 => SET(0, Reg8(L)),
      0xC6 => SET(0, Mem(HL)),
      0xC7 => SET(0, Reg8(A)),
      0xC8 => SET(1, Reg8(B)),
      0xC9 => SET(1, Reg8(C)),
      0xCA => SET(1, Reg8(D)),
      0xCB => SET(1, Reg8(E)),
      0xCC => SET(1, Reg8(H)),
      0xCD => SET(1, Reg8(L)),
      0xCE => SET(1, Mem(HL)),
      0xCF => SET(1, Reg8(A)),
      0xD0 => SET(2, Reg8(B)),
      0xD1 => SET(2, Reg8(C)),
      0xD2 => SET(2, Reg8(D)),
      0xD3 => SET(2, Reg8(E)),
      0xD4 => SET(2, Reg8(H)),
      0xD5 => SET(2, Reg8(L)),
      0xD6 => SET(2, Mem(HL)),
      0xD7 => SET(2, Reg8(A)),
      0xD8 => SET(3, Reg8(B)),
      0xD9 => SET(3, Reg8(C)),
      0xDA => SET(3, Reg8(D)),
      0xDB => SET(3, Reg8(E)),
      0xDC => SET(3, Reg8(H)),
      0xDD => SET(3, Reg8(L)),
      0xDE => SET(3, Mem(HL)),
      0xDF => SET(3, Reg8(A)),
      0xE0 => SET(4, Reg8(B)),
      0xE1 => SET(4, Reg8(C)),
      0xE2 => SET(4, Reg8(D)),
      0xE3 => SET(4, Reg8(E)),
      0xE4 => SET(4, Reg8(H)),
      0xE5 => SET(4, Reg8(L)),
      0xE6 => SET(4, Mem(HL)),
      0xE7 => SET(4, Reg8(A)),
      0xE8 => SET(5, Reg8(B)),
      0xE9 => SET(5, Reg8(C)),
      0xEA => SET(5, Reg8(D)),
      0xEB => SET(5, Reg8(E)),
      0xEC => SET(5, Reg8(H)),
      0xED => SET(5, Reg8(L)),
      0xEE => SET(5, Mem(HL)),
      0xEF => SET(5, Reg8(A)),
      0xF0 => SET(6, Reg8(B)),
      0xF1 => SET(6, Reg8(C)),
      0xF2 => SET(6, Reg8(D)),
      0xF3 => SET(6, Reg8(E)),
      0xF4 => SET(6, Reg8(H)),
      0xF5 => SET(6, Reg8(L)),
      0xF6 => SET(6, Mem(HL)),
      0xF7 => SET(6, Reg8(A)),
      0xF8 => SET(7, Reg8(B)),
      0xF9 => SET(7, Reg8(C)),
      0xFA => SET(7, Reg8(D)),
      0xFB => SET(7, Reg8(E)),
      0xFC => SET(7, Reg8(H)),
      0xFD => SET(7, Reg8(L)),
      0xFE => SET(7, Mem(HL)),
      0xFF => SET(7, Reg8(A)),
  }
}