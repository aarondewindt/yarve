
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
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31
}

impl From<u32> for Register {
    fn from(value: u32) -> Register {
        Register::from(value as usize)
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Register {
        match value {
            0 => Register::X0,
            1 => Register::X1,
            2 => Register::X2,
            3 => Register::X3,
            4 => Register::X4,
            5 => Register::X5,
            6 => Register::X6,
            7 => Register::X7,
            8 => Register::X8,
            9 => Register::X9,
            10 => Register::X10,
            11 => Register::X11,
            12 => Register::X12,
            13 => Register::X13,
            14 => Register::X14,
            15 => Register::X15,
            16 => Register::X16,
            17 => Register::X17,
            18 => Register::X18,
            19 => Register::X19,
            20 => Register::X20,
            21 => Register::X21,
            22 => Register::X22,
            23 => Register::X23,
            24 => Register::X24,
            25 => Register::X25,
            26 => Register::X26,
            27 => Register::X27,
            28 => Register::X28,
            29 => Register::X29,
            30 => Register::X30,
            31 => Register::X31,
            _ => panic!("Invalid register value.")
        }
    }
}

impl From<Register> for usize {
    fn from(register: Register) -> usize {
        match register {
            Register::X0 => 0,
            Register::X1 => 1,
            Register::X2 => 2,
            Register::X3 => 3,
            Register::X4 => 4,
            Register::X5 => 5,
            Register::X6 => 6,
            Register::X7 => 7,
            Register::X8 => 8,
            Register::X9 => 9,
            Register::X10 => 10,
            Register::X11 => 11,
            Register::X12 => 12,
            Register::X13 => 13,
            Register::X14 => 14,
            Register::X15 => 15,
            Register::X16 => 16,
            Register::X17 => 17,
            Register::X18 => 18,
            Register::X19 => 19,
            Register::X20 => 20,
            Register::X21 => 21,
            Register::X22 => 22,
            Register::X23 => 23,
            Register::X24 => 24,
            Register::X25 => 25,
            Register::X26 => 26,
            Register::X27 => 27,
            Register::X28 => 28,
            Register::X29 => 29,
            Register::X30 => 30,
            Register::X31 => 31,
        }
    }
}

impl Register {
    pub const  ZERO: Register = Register::X0;
    pub const  RA: Register = Register::X1;
    pub const  SP: Register = Register::X2;
    pub const  GP: Register = Register::X3;
    pub const  TP: Register = Register::X4;
    pub const  T0: Register = Register::X5;
    pub const  T1: Register = Register::X6;
    pub const  T2: Register = Register::X7;
    pub const  S0: Register = Register::X8;
    pub const  FP: Register = Register::X8;
    pub const  S1: Register = Register::X9;
    pub const  A0: Register = Register::X10;
    pub const  A1: Register = Register::X11;
    pub const  A2: Register = Register::X12;
    pub const  A3: Register = Register::X13;
    pub const  A4: Register = Register::X14;
    pub const  A5: Register = Register::X15;
    pub const  A6: Register = Register::X16;
    pub const  A7: Register = Register::X17;
    pub const  S2: Register = Register::X18;
    pub const  S3: Register = Register::X19;
    pub const  S4: Register = Register::X20;
    pub const  S5: Register = Register::X21;
    pub const  S6: Register = Register::X22;
    pub const  S7: Register = Register::X23;
    pub const  S8: Register = Register::X24;
    pub const  S9: Register = Register::X25;
    pub const  S10: Register = Register::X26;
    pub const  S11: Register = Register::X27;
    pub const  T3: Register = Register::X28;
    pub const  T4: Register = Register::X29;
    pub const  T5: Register = Register::X30;
    pub const  T6: Register = Register::X31;
}


#[derive(Copy, Clone, Debug)]
pub enum FloatRegister {
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31
}


impl FloatRegister {
    pub const FT0:FloatRegister = FloatRegister::F0;
    pub const FT1:FloatRegister = FloatRegister::F1;
    pub const FT2:FloatRegister = FloatRegister::F2;
    pub const FT3:FloatRegister = FloatRegister::F3;
    pub const FT4:FloatRegister = FloatRegister::F4;
    pub const FT5:FloatRegister = FloatRegister::F5;
    pub const FT6:FloatRegister = FloatRegister::F6;
    pub const FT7:FloatRegister = FloatRegister::F7;
    pub const FS0:FloatRegister = FloatRegister::F8;
    pub const FS1:FloatRegister = FloatRegister::F9;
    pub const FA0:FloatRegister = FloatRegister::F10;
    pub const FA1:FloatRegister = FloatRegister::F11;
    pub const FA2:FloatRegister = FloatRegister::F12;
    pub const FA3:FloatRegister = FloatRegister::F13;
    pub const FA4:FloatRegister = FloatRegister::F14;
    pub const FA5:FloatRegister = FloatRegister::F15;
    pub const FA6:FloatRegister = FloatRegister::F16;
    pub const FA7:FloatRegister = FloatRegister::F17;
    pub const FS2:FloatRegister = FloatRegister::F18;
    pub const FS3:FloatRegister = FloatRegister::F19;
    pub const FS4:FloatRegister = FloatRegister::F20;
    pub const FS5:FloatRegister = FloatRegister::F21;
    pub const FS6:FloatRegister = FloatRegister::F22;
    pub const FS7:FloatRegister = FloatRegister::F23;
    pub const FS8:FloatRegister = FloatRegister::F24;
    pub const FS9:FloatRegister = FloatRegister::F25;
    pub const FS10:FloatRegister = FloatRegister::F26;
    pub const FS11:FloatRegister = FloatRegister::F27;
    pub const FT8:FloatRegister = FloatRegister::F28;
    pub const FT9:FloatRegister = FloatRegister::F29;
    pub const FT10:FloatRegister = FloatRegister::F30;
    pub const FT11:FloatRegister = FloatRegister::F31;
}

impl From<usize> for FloatRegister {
    fn from(value: usize) -> FloatRegister {
        match value {
            0 => FloatRegister::F0,
            1 => FloatRegister::F1,
            2 => FloatRegister::F2,
            3 => FloatRegister::F3,
            4 => FloatRegister::F4,
            5 => FloatRegister::F5,
            6 => FloatRegister::F6,
            7 => FloatRegister::F7,
            8 => FloatRegister::F8,
            9 => FloatRegister::F9,
            10 => FloatRegister::F10,
            11 => FloatRegister::F11,
            12 => FloatRegister::F12,
            13 => FloatRegister::F13,
            14 => FloatRegister::F14,
            15 => FloatRegister::F15,
            16 => FloatRegister::F16,
            17 => FloatRegister::F17,
            18 => FloatRegister::F18,
            19 => FloatRegister::F19,
            20 => FloatRegister::F20,
            21 => FloatRegister::F21,
            22 => FloatRegister::F22,
            23 => FloatRegister::F23,
            24 => FloatRegister::F24,
            25 => FloatRegister::F25,
            26 => FloatRegister::F26,
            27 => FloatRegister::F27,
            28 => FloatRegister::F28,
            29 => FloatRegister::F29,
            30 => FloatRegister::F30,
            31 => FloatRegister::F31,
            _ => panic!("Invalid float register value.")
        }
    }
}

impl From<FloatRegister> for usize {
    fn from(register: FloatRegister) -> usize {
        match register {
            FloatRegister::F0 => 0,
            FloatRegister::F1 => 1,
            FloatRegister::F2 => 2,
            FloatRegister::F3 => 3,
            FloatRegister::F4 => 4,
            FloatRegister::F5 => 5,
            FloatRegister::F6 => 6,
            FloatRegister::F7 => 7,
            FloatRegister::F8 => 8,
            FloatRegister::F9 => 9,
            FloatRegister::F10 => 10,
            FloatRegister::F11 => 11,
            FloatRegister::F12 => 12,
            FloatRegister::F13 => 13,
            FloatRegister::F14 => 14,
            FloatRegister::F15 => 15,
            FloatRegister::F16 => 16,
            FloatRegister::F17 => 17,
            FloatRegister::F18 => 18,
            FloatRegister::F19 => 19,
            FloatRegister::F20 => 20,
            FloatRegister::F21 => 21,
            FloatRegister::F22 => 22,
            FloatRegister::F23 => 23,
            FloatRegister::F24 => 24,
            FloatRegister::F25 => 25,
            FloatRegister::F26 => 26,
            FloatRegister::F27 => 27,
            FloatRegister::F28 => 28,
            FloatRegister::F29 => 29,
            FloatRegister::F30 => 30,
            FloatRegister::F31 => 31,
        }
    }
}
