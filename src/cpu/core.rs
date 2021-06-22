use crate::cpu::register::{Register, FloatRegister};


pub struct _Core {
    pc: u64,
    registers: [u64; 32],
    float_registers: [f64; 32],
}

impl _Core {
    fn _get_register(&self, register: Register) -> u64 {
        match register {
            Register::x0 => 0,
            _ => self.registers[register as usize],
        }
    }

    fn _set_register(&mut self, register: Register, value: u64) {
        match register {
            Register::x0 => {},
            _ => self.registers[register as usize] = value,
        }
    }

    fn _get_float_register(&self, register: FloatRegister) -> f64 {
        self.float_registers[register as usize]
    }

    fn _set_float_register(&mut self, register: Register, value: f64) {
        self.float_registers[register as usize] = value;
    }
}
