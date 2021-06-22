
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


#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Register {
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

impl From<u32> for Register {
    fn from(value: u32) -> Register {
        Register::from(value as usize)
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Register {
        match value {
            0 => Register::x0,
            1 => Register::x1,
            2 => Register::x2,
            3 => Register::x3,
            4 => Register::x4,
            5 => Register::x5,
            6 => Register::x6,
            7 => Register::x7,
            8 => Register::x8,
            9 => Register::x9,
            10 => Register::x10,
            11 => Register::x11,
            12 => Register::x12,
            13 => Register::x13,
            14 => Register::x14,
            15 => Register::x15,
            16 => Register::x16,
            17 => Register::x17,
            18 => Register::x18,
            19 => Register::x19,
            20 => Register::x20,
            21 => Register::x21,
            22 => Register::x22,
            23 => Register::x23,
            24 => Register::x24,
            25 => Register::x25,
            26 => Register::x26,
            27 => Register::x27,
            28 => Register::x28,
            29 => Register::x29,
            30 => Register::x30,
            31 => Register::x31,
            _ => panic!("Invalid register value.")
        }
    }
}

impl From<Register> for usize {
    fn from(register: Register) -> usize {
        match register {
            Register::x0 => 0,
            Register::x1 => 1,
            Register::x2 => 2,
            Register::x3 => 3,
            Register::x4 => 4,
            Register::x5 => 5,
            Register::x6 => 6,
            Register::x7 => 7,
            Register::x8 => 8,
            Register::x9 => 9,
            Register::x10 => 10,
            Register::x11 => 11,
            Register::x12 => 12,
            Register::x13 => 13,
            Register::x14 => 14,
            Register::x15 => 15,
            Register::x16 => 16,
            Register::x17 => 17,
            Register::x18 => 18,
            Register::x19 => 19,
            Register::x20 => 20,
            Register::x21 => 21,
            Register::x22 => 22,
            Register::x23 => 23,
            Register::x24 => 24,
            Register::x25 => 25,
            Register::x26 => 26,
            Register::x27 => 27,
            Register::x28 => 28,
            Register::x29 => 29,
            Register::x30 => 30,
            Register::x31 => 31,
        }
    }
}

#[allow(non_upper_case_globals)]
impl Register {
    pub const  zero: Register = Register::x0;
    pub const  rA: Register = Register::x1;
    pub const  sP: Register = Register::x2;
    pub const  gP: Register = Register::x3;
    pub const  tP: Register = Register::x4;
    pub const  t0: Register = Register::x5;
    pub const  t1: Register = Register::x6;
    pub const  t2: Register = Register::x7;
    pub const  s0: Register = Register::x8;
    pub const  fP: Register = Register::x8;
    pub const  s1: Register = Register::x9;
    pub const  a0: Register = Register::x10;
    pub const  a1: Register = Register::x11;
    pub const  a2: Register = Register::x12;
    pub const  a3: Register = Register::x13;
    pub const  a4: Register = Register::x14;
    pub const  a5: Register = Register::x15;
    pub const  a6: Register = Register::x16;
    pub const  a7: Register = Register::x17;
    pub const  s2: Register = Register::x18;
    pub const  s3: Register = Register::x19;
    pub const  s4: Register = Register::x20;
    pub const  s5: Register = Register::x21;
    pub const  s6: Register = Register::x22;
    pub const  s7: Register = Register::x23;
    pub const  s8: Register = Register::x24;
    pub const  s9: Register = Register::x25;
    pub const  s10: Register = Register::x26;
    pub const  s11: Register = Register::x27;
    pub const  t3: Register = Register::x28;
    pub const  t4: Register = Register::x29;
    pub const  t5: Register = Register::x30;
    pub const  t6: Register = Register::x31;
}


#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum FloatRegister {
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

#[allow(non_upper_case_globals)]
impl FloatRegister {
    pub const ft0:FloatRegister = FloatRegister::f0;
    pub const ft1:FloatRegister = FloatRegister::f1;
    pub const ft2:FloatRegister = FloatRegister::f2;
    pub const ft3:FloatRegister = FloatRegister::f3;
    pub const ft4:FloatRegister = FloatRegister::f4;
    pub const ft5:FloatRegister = FloatRegister::f5;
    pub const ft6:FloatRegister = FloatRegister::f6;
    pub const ft7:FloatRegister = FloatRegister::f7;
    pub const fs0:FloatRegister = FloatRegister::f8;
    pub const fs1:FloatRegister = FloatRegister::f9;
    pub const fa0:FloatRegister = FloatRegister::f10;
    pub const fa1:FloatRegister = FloatRegister::f11;
    pub const fa2:FloatRegister = FloatRegister::f12;
    pub const fa3:FloatRegister = FloatRegister::f13;
    pub const fa4:FloatRegister = FloatRegister::f14;
    pub const fa5:FloatRegister = FloatRegister::f15;
    pub const fa6:FloatRegister = FloatRegister::f16;
    pub const fa7:FloatRegister = FloatRegister::f17;
    pub const fs2:FloatRegister = FloatRegister::f18;
    pub const fs3:FloatRegister = FloatRegister::f19;
    pub const fs4:FloatRegister = FloatRegister::f20;
    pub const fs5:FloatRegister = FloatRegister::f21;
    pub const fs6:FloatRegister = FloatRegister::f22;
    pub const fs7:FloatRegister = FloatRegister::f23;
    pub const fs8:FloatRegister = FloatRegister::f24;
    pub const fs9:FloatRegister = FloatRegister::f25;
    pub const fs10:FloatRegister = FloatRegister::f26;
    pub const fs11:FloatRegister = FloatRegister::f27;
    pub const ft8:FloatRegister = FloatRegister::f28;
    pub const ft9:FloatRegister = FloatRegister::f29;
    pub const ft10:FloatRegister = FloatRegister::f30;
    pub const ft11:FloatRegister = FloatRegister::f31;
}

impl From<usize> for FloatRegister {
    fn from(value: usize) -> FloatRegister {
        match value {
            0 => FloatRegister::f0,
            1 => FloatRegister::f1,
            2 => FloatRegister::f2,
            3 => FloatRegister::f3,
            4 => FloatRegister::f4,
            5 => FloatRegister::f5,
            6 => FloatRegister::f6,
            7 => FloatRegister::f7,
            8 => FloatRegister::f8,
            9 => FloatRegister::f9,
            10 => FloatRegister::f10,
            11 => FloatRegister::f11,
            12 => FloatRegister::f12,
            13 => FloatRegister::f13,
            14 => FloatRegister::f14,
            15 => FloatRegister::f15,
            16 => FloatRegister::f16,
            17 => FloatRegister::f17,
            18 => FloatRegister::f18,
            19 => FloatRegister::f19,
            20 => FloatRegister::f20,
            21 => FloatRegister::f21,
            22 => FloatRegister::f22,
            23 => FloatRegister::f23,
            24 => FloatRegister::f24,
            25 => FloatRegister::f25,
            26 => FloatRegister::f26,
            27 => FloatRegister::f27,
            28 => FloatRegister::f28,
            29 => FloatRegister::f29,
            30 => FloatRegister::f30,
            31 => FloatRegister::f31,
            _ => panic!("Invalid float register value.")
        }
    }
}

impl From<FloatRegister> for usize {
    fn from(register: FloatRegister) -> usize {
        match register {
            FloatRegister::f0 => 0,
            FloatRegister::f1 => 1,
            FloatRegister::f2 => 2,
            FloatRegister::f3 => 3,
            FloatRegister::f4 => 4,
            FloatRegister::f5 => 5,
            FloatRegister::f6 => 6,
            FloatRegister::f7 => 7,
            FloatRegister::f8 => 8,
            FloatRegister::f9 => 9,
            FloatRegister::f10 => 10,
            FloatRegister::f11 => 11,
            FloatRegister::f12 => 12,
            FloatRegister::f13 => 13,
            FloatRegister::f14 => 14,
            FloatRegister::f15 => 15,
            FloatRegister::f16 => 16,
            FloatRegister::f17 => 17,
            FloatRegister::f18 => 18,
            FloatRegister::f19 => 19,
            FloatRegister::f20 => 20,
            FloatRegister::f21 => 21,
            FloatRegister::f22 => 22,
            FloatRegister::f23 => 23,
            FloatRegister::f24 => 24,
            FloatRegister::f25 => 25,
            FloatRegister::f26 => 26,
            FloatRegister::f27 => 27,
            FloatRegister::f28 => 28,
            FloatRegister::f29 => 29,
            FloatRegister::f30 => 30,
            FloatRegister::f31 => 31,
        }
    }
}
