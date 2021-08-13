use std::fmt::{Display, Formatter, Debug};
use std::error::Error;

use crate::cpu::instruction::Instruction;
use crate::cpu::core::Core;
use crate::device::{Device, DeviceError};
use crate::endianness::Endianness;


#[derive(Debug)]
pub enum InstructionExecuteError {
    NotImplemented(Instruction),
    DeviceError(DeviceError),
}

impl Display for InstructionExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InstructionExecuteError { }

impl From<DeviceError> for InstructionExecuteError {
    fn from(error: DeviceError) -> Self {
        InstructionExecuteError::DeviceError(error)
    }
}


impl Instruction {
    pub fn execute(&self, core: &mut Core) -> Result<(), InstructionExecuteError> {
        match self {
            // RV32I & RV64I Base Integer Instructions
            // RV32I integer arithmetic instructions
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
                core.x_registers[*rd] = core.x_registers[*rs1] >> (core.x_registers[*rs2] as i64);
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

            Instruction::addi {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1];
                core.x_registers[*rd] = rs1.wrapping_add(*imm as u64);
            },

            Instruction::xori {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] ^ imm;
            },

            Instruction::ori {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] | imm;
            },

            Instruction::andi {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] & imm;
            },

            Instruction::slli {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] << shamt;
            },

            Instruction::srli {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> shamt;
            },

            Instruction::srai {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> shamt;
            },

            Instruction::slti {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                core.x_registers[*rd] = if rs1 < *imm {1} else {0};
            },

            Instruction::sltiu {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1];
                core.x_registers[*rd] = if rs1 < *imm {1} else {0};
            },

            // Load instructions 32 + 64
            Instruction::lb {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 1, Endianness::LittleEndian, true)?
            }

            Instruction::lh {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 2, Endianness::LittleEndian, true)?
            }

            Instruction::lw {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 4, Endianness::LittleEndian, true)?
            }

            Instruction::ld {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 8, Endianness::LittleEndian, false)?
            }

            Instruction::lbu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 1, Endianness::LittleEndian, false)?
            }

            Instruction::lhu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 2, Endianness::LittleEndian, false)?
            }

            Instruction::lwu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 4, Endianness::LittleEndian, false)?
            }

            // Store instructions 32 + 64
            Instruction::sb {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 1, Endianness::LittleEndian)?
            }

            Instruction::sh {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 2, Endianness::LittleEndian)?
            }

            Instruction::sw {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 4, Endianness::LittleEndian)?
            }

            Instruction::sd {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 8, Endianness::LittleEndian)?
            }

            // Branching
            Instruction::beq {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 == rs2 { core.add_to_pc(*imm) };
            },

            Instruction::bne {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 != rs2 { core.add_to_pc(*imm) };
            },

            Instruction::blt {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                if rs1 < rs2 { core.add_to_pc(*imm) };
            },

            Instruction::bge {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                if rs1 >= rs2 { core.add_to_pc(*imm) };
            },

            Instruction::bltu {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 < rs2 { core.add_to_pc(*imm) };
            },

            Instruction::bgeu {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 >= rs2 { core.add_to_pc(*imm) };
            },

            Instruction::jal {rd, imm} => {
                core.x_registers[*rd] = core.pc as u64 + 4;
                core.add_to_pc(*imm);
            },

            Instruction::jalr {rd,rs1, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                core.x_registers[*rd] = core.pc as u64 + 4;
                core.add_to_pc(rs1 + *imm);
            },

            Instruction::lui {rd, uimm} => {
                core.x_registers[*rd] = *uimm;
            },

            Instruction::auipc {rd, imm} => {
                core.x_registers[*rd] = core.pc.wrapping_add(*imm as usize) as u64;
            },

            // Ignore these instructions for now.
            Instruction::ecall | Instruction::ebreak
            | Instruction::fence | Instruction::fence_tso => {},

            _ => {
                return Err(InstructionExecuteError::NotImplemented(*self))
            }
        }

        Ok(())
    }
}
