use crate::cpu::register::{XRegister, FRegister};


use enum_map::EnumMap;
use std::ops::{Index, IndexMut};


pub struct CoreXRegisters {
    registers: EnumMap<XRegister, u64>,
}

pub struct CoreFRegisters {
    registers: EnumMap<FRegister, f64>,
}


impl CoreXRegisters {
    fn new() -> CoreXRegisters {
        CoreXRegisters {
            registers: enum_map! {
                XRegister::x0 => 0, XRegister::x1 => 0, XRegister::x2 => 0, XRegister::x3 => 0,
                XRegister::x4 => 0, XRegister::x5 => 0, XRegister::x6 => 0, XRegister::x7 => 0,
                XRegister::x8 => 0, XRegister::x9 => 0, XRegister::x10 => 0, XRegister::x11 => 0,
                XRegister::x12 => 0, XRegister::x13 => 0, XRegister::x14 => 0, XRegister::x15 => 0,
                XRegister::x16 => 0, XRegister::x17 => 0, XRegister::x18 => 0, XRegister::x19 => 0,
                XRegister::x20 => 0, XRegister::x21 => 0, XRegister::x22 => 0, XRegister::x23 => 0,
                XRegister::x24 => 0, XRegister::x25 => 0, XRegister::x26 => 0, XRegister::x27 => 0,
                XRegister::x28 => 0, XRegister::x29 => 0, XRegister::x30 => 0, XRegister::x31 => 0,
            },
        }
    }
}

impl CoreFRegisters {
    fn new() -> CoreFRegisters {
        CoreFRegisters {
            registers: enum_map! {
                FRegister::f0 => 0., FRegister::f1 => 0., FRegister::f2 => 0.,
                FRegister::f3 => 0., FRegister::f4 => 0., FRegister::f5 => 0.,
                FRegister::f6 => 0., FRegister::f7 => 0., FRegister::f8 => 0.,
                FRegister::f9 => 0., FRegister::f10 => 0., FRegister::f11 => 0.,
                FRegister::f12 => 0., FRegister::f13 => 0., FRegister::f14 => 0.,
                FRegister::f15 => 0., FRegister::f16 => 0., FRegister::f17 => 0.,
                FRegister::f18 => 0., FRegister::f19 => 0., FRegister::f20 => 0.,
                FRegister::f21 => 0., FRegister::f22 => 0., FRegister::f23 => 0.,
                FRegister::f24 => 0., FRegister::f25 => 0., FRegister::f26 => 0.,
                FRegister::f27 => 0., FRegister::f28 => 0., FRegister::f29 => 0.,
                FRegister::f30 => 0., FRegister::f31 => 0.
            }
        }
    }
}

impl Index<XRegister> for CoreXRegisters {
    type Output = u64;
    fn index(&self, register: XRegister) -> &Self::Output {
        if register == XRegister::x0 {&0} else {&self.registers[register]}
    }
}

impl Index<FRegister> for CoreFRegisters {
    type Output = f64;
    fn index(&self, register: FRegister) -> &Self::Output {
        &self.registers[register]
    }
}

impl IndexMut<XRegister> for CoreXRegisters {
    fn index_mut(&mut self, register: XRegister) -> &mut Self::Output {
        &mut self.registers[register]
    }
}

impl IndexMut<FRegister> for CoreFRegisters {
    fn index_mut(&mut self, register: FRegister) -> &mut Self::Output {
        &mut self.registers[register]
    }
}

pub struct Core {
    pub pc: u64,
    pub x_registers: CoreXRegisters,
    pub f_registers: CoreFRegisters
}

impl Core {
    pub fn new() -> Core {
        Core {
            pc: 0,
            x_registers: CoreXRegisters::new(),
            f_registers: CoreFRegisters::new()
        }
    }
}
