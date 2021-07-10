use crate::cpu::register::{Register, FloatRegister};


use enum_map::EnumMap;


pub struct Core {
    pc: u64,
    registers: EnumMap<Register, u64>,
    float_registers: EnumMap<FloatRegister, u64>,
}

impl Core {
    fn new() -> Core {
        Core{
            pc: 0,
            registers: enum_map! {
                Register::x0 => 0, Register::x1 => 0, Register::x2 => 0, Register::x3 => 0,
                Register::x4 => 0, Register::x5 => 0, Register::x6 => 0, Register::x7 => 0,
                Register::x8 => 0, Register::x9 => 0, Register::x10 => 0, Register::x11 => 0,
                Register::x12 => 0, Register::x13 => 0, Register::x14 => 0, Register::x15 => 0,
                Register::x16 => 0, Register::x17 => 0, Register::x18 => 0, Register::x19 => 0,
                Register::x20 => 0, Register::x21 => 0, Register::x22 => 0, Register::x23 => 0,
                Register::x24 => 0, Register::x25 => 0, Register::x26 => 0, Register::x27 => 0,
                Register::x28 => 0, Register::x29 => 0, Register::x30 => 0, Register::x31 => 0,
            },
            float_registers: enum_map! {
                FloatRegister::f0 => 0, FloatRegister::f1 => 0, FloatRegister::f2 => 0,
                FloatRegister::f3 => 0, FloatRegister::f4 => 0, FloatRegister::f5 => 0,
                FloatRegister::f6 => 0, FloatRegister::f7 => 0, FloatRegister::f8 => 0,
                FloatRegister::f9 => 0, FloatRegister::f10 => 0, FloatRegister::f11 => 0,
                FloatRegister::f12 => 0, FloatRegister::f13 => 0, FloatRegister::f14 => 0,
                FloatRegister::f15 => 0, FloatRegister::f16 => 0, FloatRegister::f17 => 0,
                FloatRegister::f18 => 0, FloatRegister::f19 => 0, FloatRegister::f20 => 0,
                FloatRegister::f21 => 0, FloatRegister::f22 => 0, FloatRegister::f23 => 0,
                FloatRegister::f24 => 0, FloatRegister::f25 => 0, FloatRegister::f26 => 0,
                FloatRegister::f27 => 0, FloatRegister::f28 => 0, FloatRegister::f29 => 0,
                FloatRegister::f30 => 0, FloatRegister::f31 => 0,
            }
        }
    }
}

