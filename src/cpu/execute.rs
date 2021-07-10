use crate::cpu::instruction::{Instruction, RoundingMode, FloatFormat, InstructionFormat};
use crate::cpu::register::{XRegister, FRegister};
use crate::cpu::core::Core;
use crate::cpu::execute::InstructionExecuteError::NotImplemented;


#[derive(Debug)]
pub enum InstructionExecuteError {
    NotImplemented
}


impl Instruction {
    pub fn execute(&self) -> Result<(), InstructionExecuteError> {
        match self {
            Instruction::add{rd, rs1,rs2} => {
                Ok(())
            },

            _ => {
                Err(NotImplemented)
            }
        }
    }
}
