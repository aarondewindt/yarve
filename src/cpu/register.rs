use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
    x31
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
