use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use enum_map::EnumMap;
use std::ops::{Index, IndexMut};

// RISK-V registers.
// x0       zero    Hard-wired zero —
// x1       ra      Return address Caller
// x2       sp      Stack pointer Callee
// x3       gp      Global pointer —
// x4       tp      Thread pointer —
// x5–7     t0–2    Temporaries Caller
// x8       s0/fp   Saved register/frame pointer Callee
// x9       s1      Saved register Callee
// x10–11   a0–1    Function arguments/return values Caller
// x12–17   a2–7    Function arguments Caller
// x18–27   s2–11   Saved registers Callee
// x28–31   t3–6    Temporaries Caller
// f0–7     ft0–7   FP temporaries Caller
// f8–9     fs0–1   FP saved registers Callee
// f10–11   fa0–1   FP arguments/return values Caller
// f12–17   fa2–7   FP arguments Caller
// f18–27   fs2–11  FP saved registers Callee
// f28–31   ft8–11  FP temporaries Caller


#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Copy, Clone, Enum)]
#[allow(non_camel_case_types)]
pub enum XRegister {
    x0,
    x1,
    x2,
    x3,
    x4,
    x5,
    x6,
    x7,
    x8,
    x9,
    x10,
    x11,
    x12,
    x13,
    x14,
    x15,
    x16,
    x17,
    x18,
    x19,
    x20,
    x21,
    x22,
    x23,
    x24,
    x25,
    x26,
    x27,
    x28,
    x29,
    x30,
    x31,
}

#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, Copy, Clone, Enum)]
#[allow(non_camel_case_types)]
pub enum FRegister {
    f0,
    f1,
    f2,
    f3,
    f4,
    f5,
    f6,
    f7,
    f8,
    f9,
    f10,
    f11,
    f12,
    f13,
    f14,
    f15,
    f16,
    f17,
    f18,
    f19,
    f20,
    f21,
    f22,
    f23,
    f24,
    f25,
    f26,
    f27,
    f28,
    f29,
    f30,
    f31
}

impl From<u32> for XRegister {
    fn from(value: u32) -> XRegister {
        FromPrimitive::from_u32(value).unwrap()
    }
}

impl From<u32> for FRegister {
    fn from(value: u32) -> FRegister {
        FromPrimitive::from_u32(value).unwrap()
    }
}

impl From<u64> for FRegister {
    fn from(value: u64) -> FRegister {
        FromPrimitive::from_u64(value).unwrap()
    }
}


pub struct XRegisterMap {
    registers: EnumMap<XRegister, u64>,
}

pub struct FRegisterMap {
    registers: EnumMap<FRegister, f64>,
}

impl XRegisterMap {
    pub fn new() -> XRegisterMap {
        XRegisterMap {
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

impl FRegisterMap {
    pub fn new() -> FRegisterMap {
        FRegisterMap {
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

impl Index<XRegister> for XRegisterMap {
    type Output = u64;
    fn index(&self, register: XRegister) -> &Self::Output {
        if register == XRegister::x0 {&0} else {&self.registers[register]}
    }
}

impl Index<FRegister> for FRegisterMap {
    type Output = f64;
    fn index(&self, register: FRegister) -> &Self::Output {
        &self.registers[register]
    }
}

impl IndexMut<XRegister> for XRegisterMap {
    fn index_mut(&mut self, register: XRegister) -> &mut Self::Output {
        &mut self.registers[register]
    }
}

impl IndexMut<FRegister> for FRegisterMap {
    fn index_mut(&mut self, register: FRegister) -> &mut Self::Output {
        &mut self.registers[register]
    }
}
