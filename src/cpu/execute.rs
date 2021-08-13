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
        let advance_pc = match self {
            // RV32I & RV64I Base Integer Instructions
            // RV32I integer arithmetic instructions
            Instruction::add{rd, rs1,rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1]
                    .wrapping_add(core.x_registers[*rs2]);
                true
            },

            Instruction::sub{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1]
                    .wrapping_sub(core.x_registers[*rs2]);
                true
            },

            Instruction::xor{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] ^ core.x_registers[*rs2];
                true
            },

            Instruction::or{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] | core.x_registers[*rs2];
                true
            },

            Instruction::and{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] & core.x_registers[*rs2];
                true
            },

            Instruction::sll{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] << core.x_registers[*rs2];
                true
            },

            Instruction::srl{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> core.x_registers[*rs2];
                true
            },

            Instruction::sra{rd, rs1, rs2} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> (core.x_registers[*rs2] as i64);
                true
            },

            Instruction::slt{rd, rs1, rs2} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                core.x_registers[*rd] = if rs1 < rs2 {1} else {0};
                true
            },

            Instruction::sltu{rd, rs1, rs2} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                core.x_registers[*rd] = if rs1 < rs2 {1} else {0};
                true
            },

            Instruction::addi {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1];
                core.x_registers[*rd] = rs1.wrapping_add(*imm as u64);
                true
            },

            Instruction::xori {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] ^ imm;
                true
            },

            Instruction::ori {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] | imm;
                true
            },

            Instruction::andi {rd, rs1, imm} => {
                core.x_registers[*rd] = core.x_registers[*rs1] & imm;
                true
            },

            Instruction::slli {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] << shamt;
                true
            },

            Instruction::srli {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> shamt;
                true
            },

            Instruction::srai {rd, rs1, shamt} => {
                core.x_registers[*rd] = core.x_registers[*rs1] >> shamt;
                true
            },

            Instruction::slti {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                core.x_registers[*rd] = if rs1 < *imm {1} else {0};
                true
            },

            Instruction::sltiu {rd, rs1, imm} => {
                let rs1 = core.x_registers[*rs1];
                core.x_registers[*rd] = if rs1 < *imm {1} else {0};
                true
            },

            // Load instructions 32 + 64
            Instruction::lb {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 1, Endianness::LittleEndian, true)?;
                true
            }

            Instruction::lh {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 2, Endianness::LittleEndian, true)?;
                true
            }

            Instruction::lw {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 4, Endianness::LittleEndian, true)?;
                true
            }

            Instruction::ld {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 8, Endianness::LittleEndian, false)?;
                true
            }

            Instruction::lbu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 1, Endianness::LittleEndian, false)?;
                true
            }

            Instruction::lhu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 2, Endianness::LittleEndian, false)?;
                true
            }

            Instruction::lwu {rd, rs1, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.x_registers[*rd] = core.bus.borrow()
                    .read_int(address, 4, Endianness::LittleEndian, false)?;
                true
            }

            // Store instructions 32 + 64
            Instruction::sb {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 1, Endianness::LittleEndian)?;
                true
            }

            Instruction::sh {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 2, Endianness::LittleEndian)?;
                true
            }

            Instruction::sw {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 4, Endianness::LittleEndian)?;
                true
            }

            Instruction::sd {rs1, rs2, imm} => {
                let address = (core.x_registers[*rs1] as i64).wrapping_add(*imm) as usize;
                core.bus.borrow_mut()
                    .write_int(address, core.x_registers[*rs2], 8, Endianness::LittleEndian)?;
                true
            }

            // Branching
            Instruction::beq {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 == rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::bne {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 != rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::blt {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                if rs1 < rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::bge {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                let rs2 = core.x_registers[*rs2] as i64;
                if rs1 >= rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::bltu {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 < rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::bgeu {rs1, rs2, imm} => {
                let rs1 = core.x_registers[*rs1];
                let rs2 = core.x_registers[*rs2];
                if rs1 >= rs2 { core.add_to_pc(*imm); false } else { true }
            },

            Instruction::jal {rd, imm} => {
                core.x_registers[*rd] = core.pc as u64 + 4;
                core.add_to_pc(*imm);
                false
            },

            Instruction::jalr {rd,rs1, imm} => {
                let rs1 = core.x_registers[*rs1] as i64;
                core.x_registers[*rd] = core.pc as u64 + 4;
                core.add_to_pc(rs1 + *imm);
                false
            },

            Instruction::lui {rd, uimm} => {
                core.x_registers[*rd] = *uimm;
                true
            },

            Instruction::auipc {rd, imm} => {
                core.x_registers[*rd] = core.pc.wrapping_add(*imm as usize) as u64;
                true
            },

            // Ignore these instructions for now.
            Instruction::ecall | Instruction::ebreak | Instruction::fence_tso => { true },
            Instruction::fence { rd, rs1, succ, pred, fm } =>
                { true },

            _ => {
                return Err(InstructionExecuteError::NotImplemented(*self))
            }
        };

        if advance_pc { core.pc += 4 };

        Ok(())
    }
}
