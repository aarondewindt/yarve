pub mod register;
mod instruction;
mod test;

pub use register::{Register};


pub struct _CPU {
    pc: u64,
    registers: [u64; 32],
    float_registers: [f64; 32],
}

impl _CPU {
    fn _get_register(&self, register: Register) -> u64 {
        match register {
            Register::X0 => 0,
            _ => self.registers[register as usize],
        }
    }

    fn _set_register(&mut self, register: Register, value: u64) {
        match register {
            Register::X0 => {},
            _ => self.registers[register as usize] = value,
        }
    }

    fn _get_float_register(&self, register: Register) -> u64 {
        match register {
            Register::X0 => 0,
            _ => self.registers[register as usize],
        }
    }

    fn _set_float_register(&mut self, register: Register, value: u64) {
        match register {
            Register::X0 => {},
            _ => self.registers[register as usize] = value,
        }
    }
}


