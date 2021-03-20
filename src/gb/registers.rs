#[derive(Debug)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub flag: FlagRegister,
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
            pc: 0x0,
            sp: 0x0,
            flag: FlagRegister::new(),
        }
    }

    pub fn get_r8(&self, reg: R8) -> u8 {
        match reg {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::F => self.flag.to_u8(),
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub fn set_r8(&mut self, reg: R8, v: u8) {
        match reg {
            R8::A => self.a = v,
            R8::B => self.b = v,
            R8::C => self.c = v,
            R8::D => self.d = v,
            R8::E => self.e = v,
            R8::H => self.h = v,
            R8::L => self.l = v,
            R8::F => self.flag.set(v),
        };
    }

    pub fn get_r16(&self, reg: R16) -> u16 {
        match reg {
            R16::SP => self.sp,
            R16::PC => self.pc,
            reg => {
                let (h, l) = match reg {
                    R16::AF => (self.a, self.flag.to_u8()),
                    R16::BC => (self.b, self.c),
                    R16::DE => (self.d, self.e),
                    R16::HL => (self.h, self.l),
                    _ => panic!("invalid match"),
                };

                (h as u16) << 8 | (l as u16)
            }
        }
    }

    pub fn set_r16(&mut self, reg: R16, v: u16) {
        let h = (v >> 8) as u8;
        let l = (v & 0xFF) as u8;

        match reg {
            R16::AF => {
                self.a = h;
                self.flag.set(l);
            }
            R16::BC => {
                self.b = h;
                self.c = l;
            }
            R16::DE => {
                self.d = h;
                self.e = l;
            }
            R16::HL => {
                self.h = h;
                self.l = l;
            }
            R16::SP => {
                self.sp = v;
            }
            R16::PC => {
                self.pc = v;
            }
        };
    }
}

pub struct FlagRegister {
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            negative: false,
            half_carry: false,
            carry: false,
        }
    }

    pub fn set(&mut self, v: u8) {
        self.zero = (v & (1 << 7)) != 0;
        self.negative = (v & (1 << 6)) != 0;
        self.half_carry = (v & (1 << 5)) != 0;
        self.carry = (v & (1 << 4)) != 0;
    }

    pub fn to_u8(&self) -> u8 {
        (if self.zero { 1 << 7 } else { 0 })
            | (if self.negative { 1 << 6 } else { 0 })
            | (if self.half_carry { 1 << 5 } else { 0 })
            | (if self.carry { 1 << 4 } else { 0 })
    }
}
