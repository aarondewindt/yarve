use crate::cpu::register::Register;


#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    Undefined {instruction: u32, msg: String},

    // RV32I Base Instruction Set
    LUI {rd: Register, imm: u64},
    AUIPC {rd: Register, imm: u64},
    _JAL,
    JALR {rd: Register, rs1: Register, imm: u64},
    _BEQ,
    _BNE,
    _BLT,
    _BGE,
    _BLTU,
    _BGEU,
    LB {rd: Register, rs1: Register, imm: u64},
    LH {rd: Register, rs1: Register, imm: u64},
    LW {rd: Register, rs1: Register, imm: u64},
    LBU {rd: Register, rs1: Register, imm: u64},
    LHU {rd: Register, rs1: Register, imm: u64},
    _SB,
    _SH,
    _SW,
    ADDI {rd: Register, rs1: Register, imm: u64},
    SLTI {rd: Register, rs1: Register, imm: u64},
    SLTIU {rd: Register, rs1: Register, imm: u64},
    XORI {rd: Register, rs1: Register, imm: u64},
    ORI {rd: Register, rs1: Register, imm: u64},
    ANDI {rd: Register, rs1: Register, imm: u64},
    _ADD,
    _SUB,
    _SLL,
    _SLT,
    _SLTU,
    _XOR,
    _SRL,
    _SRA,
    _OR,
    _AND,
    _FENCE,
    _FENCE_TSO,
    _PAUSE,
    ECALL,
    EBREAK,

    // RV64I Base Instruction Set
    LWU {rd: Register, rs1: Register, imm: u64},
    LD {rd: Register, rs1: Register, imm: u64},
    _SD,
    _SLLI,
    _SRLI,
    _SRAI,
    ADDIW {rd: Register, rs1: Register, imm: u64},
    _SLLIW,
    _SRLIW,
    _SRAIW,
    _ADDW,
    _SUBW,
    _SLLW,
    _SRLW,
    _SRAW,

    // RV32/RV64 Zifencei Standard Extension
    FENCE_I {rd: Register, rs1: Register, imm: u64},

    // RV32/RV64 Zicsr Standard Extension
    CSRRW {rd: Register, rs1: Register, imm: u64},
    CSRRS {rd: Register, rs1: Register, imm: u64},
    CSRRC {rd: Register, rs1: Register, imm: u64},
    CSRRWI {rd: Register, uimm: u64, imm: u64},
    CSRRSI {rd: Register, uimm: u64, imm: u64},
    CSRRCI {rd: Register, uimm: u64, imm: u64},

    // RV32M Standard Extension
    _MUL,
    _MULH,
    _MULHSU,
    _MULHU,
    _DIV,
    _DIVU,
    _REM,
    _REMU,

    // RV64M Standard Extension
    _MULW,
    _DIVW,
    _DIVUW,
    _REMW,
    _REMUW,

    // RV32A Standard Extension
    _LR_W,
    _SC_W,
    _AMOSWAP_W,
    _AMOADD_W,
    _AMOXOR_W,
    _AMOAND_W,
    _AMOOR_W,
    _AMOMIN_W,
    _AMOMAX_W,
    _AMOMINU_W,
    _AMOMAXU_W,

    // RV64A Standard Extension
    _LR_D,
    _SC_D,
    _AMOSWAP_D,
    _AMOADD_D,
    _AMOXOR_D,
    _AMOAND_D,
    _AMOOR_D,
    _AMOMIN_D,
    _AMOMAX_D,
    _AMOMINU_D,
    _AMOMAXU_D,

    // RV32F Standard Extension
    FLW {rd: Register, rs1: Register, imm: u64},
    _FSW,
    _FMADD_S,
    _FMSUB_S,
    _FNMSUB_S,
    _FNMADD_S,
    _FADD_S,
    _FSUB_S,
    _FMUL_S,
    _FDIV_S,
    _FSQRT_S,
    _FSGNJ_S,
    _FSGNJN_S,
    _FSGNJX_S,
    _FMIN_S,
    _FMAX_S,
    _FCVT_W_S,
    _FCVT_WU_S,
    _FMV_X_W,
    _FEQ_S,
    _FLT_S,
    _FLE_S,
    _FCLASS_S,
    _FCVT_S_W,
    _FCVT_S_WU,
    _FMV_W_X,

    // RV64F Standard Extension
    _FCV_TL_S,
    _FCV_TLU_S,
    _FCV_TS_L,
    _FCV_TS_LU,

    // RV32D Standard Extension
    FLD {rd: Register, rs1: Register, imm: u64},
    _FSD,
    _FMADD_D,
    _FMSUB_D,
    _FNMSUB_D,
    _FNMADD_D,
    _FADD_D,
    _FSUB_D,
    _FMUL_D,
    _FDIV_D,
    _FSQRT_D,
    _FSGNJ_D,
    _FSGNJN_D,
    _FSGNJX_D,
    _FMIN_D,
    _FMAX_D,
    _S_D,
    _D_S,
    _FEQ_D,
    _FLT_D,
    _FLE_D,
    _FCLASS_D,
    _FCVT_W_D,
    _FCVT_WU_D,
    _FCVT_D_W,
    _FCVT_D_WU,

    // RV64D Standard Extension
    _FCVT_L_D,
    _FCVT_LU_D,
    _FMV_X_D,
    _FCVT_D_L,
    _FCVT_D_LU,
    _FMV_D_X,

}

impl Instruction {
    pub fn decode(instruction: u32) -> Instruction {
        let opcode = (instruction & 0x7F) as usize;
        match &INSTRUCTION_FORMAT_TABLE[opcode] {
            Some(format) => format.decode(instruction),
            None => Instruction::Undefined{
                instruction,
                msg: format!("No instruction format assigned to opcode {:07b}.", opcode)}
        }
    }
}

enum InstructionFormat {
    R,
    I,
    _S,
    _B,
    U,
    _J
}

impl InstructionFormat {
    pub fn decode(&self, instruction: u32) -> Instruction {
        let opcode: usize = (instruction & 0x7F) as usize;
        match self {
            InstructionFormat::I => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let func3 = (instruction >> 12) & 0b111;
                let uimm = ((instruction >> 15) & 0b11111) as u64;
                let rs1 = Register::from(uimm as usize);
                let imm =
                    0xFFFFFFFFFFFFFC00u64 * (instruction as u64 >> 31)
                    | (instruction as u64 >> 20);

                match opcode {
                    0b1100111 => {
                        match func3 {
                            0b000 => Instruction::JALR{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0000011 => {
                        match func3 {
                            0b000 => Instruction::LB{rd, rs1, imm},
                            0b001 => Instruction::LH{rd, rs1, imm},
                            0b010 => Instruction::LW{rd, rs1, imm},
                            0b100 => Instruction::LBU{rd, rs1, imm},
                            0b101 => Instruction::LHU{rd, rs1, imm},
                            0b110 => Instruction::LWU{rd, rs1, imm},
                            0b011 => Instruction::LD{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0010011 => {
                        match func3 {
                            0b000 => Instruction::ADDI{rd, rs1, imm},
                            0b010 => Instruction::SLTI{rd, rs1, imm},
                            0b011 => Instruction::SLTIU{rd, rs1, imm},
                            0b100 => Instruction::XORI{rd, rs1, imm},
                            0b110 => Instruction::ORI{rd, rs1, imm},
                            0b111 => Instruction::ANDI{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b1110011 => {
                        match func3 {
                            0b000 => {
                                match imm {
                                    0 => Instruction::ECALL,
                                    1 => Instruction::EBREAK,
                                    _ => Instruction::Undefined{
                                        instruction,
                                        msg: format!(
                                            "format: I, opcode: {:07b}, func3: {:b}, imm {:b}",
                                            opcode, func3, imm)
                                    }
                                }
                            },
                            0b001 => Instruction::CSRRW{rd, rs1, imm},
                            0b010 => Instruction::CSRRS{rd, rs1, imm},
                            0b011 => Instruction::CSRRC{rd, rs1, imm},
                            0b101 => Instruction::CSRRWI{rd, uimm, imm},
                            0b110 => Instruction::CSRRSI{rd, uimm, imm},
                            0b111 => Instruction::CSRRCI{rd, uimm, imm},
                            _ => Instruction::Undefined {
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0011011 => {
                        match func3 {
                            0b000 => Instruction::ADDIW{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0001111 => {
                        match func3 {
                            0b001 => Instruction::FENCE_I{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0000111 => {
                        match func3 {
                            0b010 => Instruction::FLW{rd, rs1, imm},
                            0b011 => Instruction::FLD{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: I, opcode: {:07b}", opcode)
                    }


                }
            },

            InstructionFormat::U => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let imm =
                    0xFFFFFFFF00000000u64 * (instruction as u64 >> 31) |
                    (instruction & 0xFFFFF800u32) as u64;

                match opcode {
                    0b0110111 => Instruction::LUI{rd, imm},
                    0b0010111 => Instruction::AUIPC{rd, imm},

                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: U, opcode: {:07b}", opcode)
                    }
                }
            },

            InstructionFormat::R => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let func3 = (instruction >> 12) & 0b111;
                let rs1 = Register::from(((instruction >> 15) & 0b11111));
                let rs2 = Register::from(((instruction >> 20) & 0b11111));
                let func7 = instruction >> 25;

                match opcode {
                    0b0110011 => {
                        match func3 {
                            0x0 => {Instruction::ADD{rd, func3, rs1, rs2, func7}},
                            0x0 => {Instruction::SUB{rd, func3, rs1, rs2, func7}},
                            0x4 => {Instruction::XOR{rd, func3, rs1, rs2, func7}},
                            0x6 => {Instruction::OR{rd, func3, rs1, rs2, func7}},
                            0x7 => {Instruction::AND{rd, func3, rs1, rs2, func7}},
                            0x1 => {Instruction::SLL{rd, func3, rs1, rs2, func7}},
                            0x5 => {Instruction::SRL{rd, func3, rs1, rs2, func7}},
                            0x5 => {Instruction::SRA{rd, func3, rs1, rs2, func7}},
                            0x2 => {Instruction::SLT{rd, func3, rs1, rs2, func7}},
                            0x3 => {Instruction::SLTU{rd, func3, rs1, rs2, func7}},
                        }
                    }
                }
            },

            InstructionFormat::J => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let imm12 = (instruction >> 12) as u64;
                let imm11 = (instruction >> 12) as u64;
                let imm1 = (instruction >> 12) as u64;
                let imm20 = (instruction >> 12) as u64;

                match opcode {
                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: J, opcode: {:07b}", opcode)
                    }
                }
            }


            _ => Instruction::Undefined{
                instruction,
                msg: format!("Format not implemented")
            }
        }
    }
}

const INSTRUCTION_FORMAT_TABLE: [Option<InstructionFormat>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(InstructionFormat::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ Some(InstructionFormat::I),
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(InstructionFormat::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(InstructionFormat::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(InstructionFormat::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ Some(InstructionFormat::I),
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ None,
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ None,
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(InstructionFormat::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ None,
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ None,
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(InstructionFormat::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ None,
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(InstructionFormat::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];
