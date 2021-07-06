use crate::cpu::instruction::{Instruction, RoundingMode, FloatFormat, InstructionFormat};
use crate::cpu::register::{Register, FloatRegister};
use crate::cpu::core::Core;


impl Instruction {
    pub fn execute(&self, &mut core: Core) {
        match self {
            Instruction::add{rd, rs1,rs2} => {

            },
            _ => {}
        }
    }
}
