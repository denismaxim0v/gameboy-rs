use super::registers::{R16, R8};

pub enum Operand {
  ImmI8,
  ImmU16,
  ImmU8,
  Mem(R16),
  MemImmU16,
  Reg16(R16),
  Reg8(R8),
  SPImmI8,
  ZMem(R8),
  ZMemImmU8
}

pub enum Condition {
  Always,
  Z,
  NZ,
  C,
  NC
}

pub enum Opcode {
  PREFIX,
  ADC(Operand, Operand),
  ADD(Operand, Operand),
  AND(Operand, Operand),
  BIT(u8, Operand),
  CALL(Condition, Operand),
  CCF,
  CP(Operand, Operand),
  CPL,
  DAA,
  DEC(Operand),
  DI,
  EI,
  HALT,
  INC(Operand),
  JP(Condition, Operand),
  JR(Condition, Operand),
  LD(Operand, Operand),
  LDD(Operand, Operand),
  LDI(Operand, Operand),
  NOP,
  OR(Operand, Operand),
  POP(Operand),
  PUSH(Operand),
  RES(u8, Operand),
  RET(Condition),
  RETI,
  RL(Operand),
  RLA,
  RLC(Operand),
  RLCA,
  RR(Operand),
  RRA,
  RRC(Operand),
  RRCA,
  RST(u8),
  SBC(Operand, Operand),
  SCF,
  SET(u8, Operand),
  SLA(Operand),
  SRA(Operand),
  SRL(Operand),
  STOP,
  SUB(Operand, Operand),
  SWAP(Operand),
  XOR(Operand, Operand)
}