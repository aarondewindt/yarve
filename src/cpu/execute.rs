use crate::cpu::instruction::{Instruction, RoundingMode, FloatFormat, InstructionFormat};
use crate::cpu::register::{XRegister, FRegister};
use crate::cpu::core::Core;


#[derive(Debug)]
pub enum InstructionExecuteError {
    NotImplemented
}


impl Instruction {
    pub fn execute(&self, core: &mut Core) -> Result<(), InstructionExecuteError> {
        match self {
            Instruction::add{rd, rs1,rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1]
                    .wrapping_add(core.x_registers[*rs2]);
            },

            Instruction::sub{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1]
                    .wrapping_sub(core.x_registers[*rs2]);
            },

            Instruction::xor{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] ^ core.x_registers[*rs2];
            },

            Instruction::or{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] | core.x_registers[*rs2];
            },

            Instruction::and{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] & core.x_registers[*rs2];
            },

            Instruction::sll{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] << core.x_registers[*rs2];
            },

            Instruction::srl{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> core.x_registers[*rs2];
            },

            Instruction::sra{rd, rs1, rs2} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                core.x_registers[*rd] = (u64::MAX << (64 - rs2)) * (rs1 >> 63) | (rs1 >> rs2);
            },

            Instruction::slt{rd, rs1, rs2} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                core.x_registers[*rd] = if rs1 < rs2 {1} else {0};
            },

            Instruction::sltu{rd, rs1, rs2} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                core.x_registers[*rd] = if rs1 < rs2 {1} else {0};
            },


            _ => {
                return Err(InstructionExecuteError::NotImplemented)
            }
        }

        Ok(())
    }
}
