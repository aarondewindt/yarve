use crate::cpu::Register;


#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, dead_code)]
pub enum Instruction {
    Undefined {instruction: u32, msg: String},

    // RV32I Base Instruction Set
    LUI {rd: Register, imm: u64},
    AUIPC {rd: Register, imm: u64},
    JAL,
    JALR {rd: Register, rs1: Register, imm: u64},
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB {rd: Register, rs1: Register, imm: u64},
    LH {rd: Register, rs1: Register, imm: u64},
    LW {rd: Register, rs1: Register, imm: u64},
    LBU {rd: Register, rs1: Register, imm: u64},
    LHU {rd: Register, rs1: Register, imm: u64},
    SB,
    SH,
    SW,
    ADDI {rd: Register, rs1: Register, imm: u64},
    SLTI {rd: Register, rs1: Register, imm: u64},
    SLTIU {rd: Register, rs1: Register, imm: u64},
    XORI {rd: Register, rs1: Register, imm: u64},
    ORI {rd: Register, rs1: Register, imm: u64},
    ANDI {rd: Register, rs1: Register, imm: u64},
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    FENCE,
    FENCE_TSO,
    PAUSE,
    ECALL,
    EBREAK,

    // RV64I Base Instruction Set
    LWU {rd: Register, rs1: Register, imm: u64},
    LD {rd: Register, rs1: Register, imm: u64},
    SD,
    SLLI,
    SRLI,
    SRAI,
    ADDIW {rd: Register, rs1: Register, imm: u64},
    SLLIW,
    SRLIW,
    SRAIW,
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,

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
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,

    // RV64M Standard Extension
    MULW,
    DIVW,
    DIVUW,
    REMW,
    REMUW,

    // RV32A Standard Extension
    LR_W,
    SC_W,
    AMOSWAP_W,
    AMOADD_W,
    AMOXOR_W,
    AMOAND_W,
    AMOOR_W,
    AMOMIN_W,
    AMOMAX_W,
    AMOMINU_W,
    AMOMAXU_W,

    // RV64A Standard Extension
    LR_D,
    SC_D,
    AMOSWAP_D,
    AMOADD_D,
    AMOXOR_D,
    AMOAND_D,
    AMOOR_D,
    AMOMIN_D,
    AMOMAX_D,
    AMOMINU_D,
    AMOMAXU_D,

    // RV32F Standard Extension
    FLW {rd: Register, rs1: Register, imm: u64},
    FSW,
    FMADD_S,
    FMSUB_S,
    FNMSUB_S,
    FNMADD_S,
    FADD_S,
    FSUB_S,
    FMUL_S,
    FDIV_S,
    FSQRT_S,
    FSGNJ_S,
    FSGNJN_S,
    FSGNJX_S,
    FMIN_S,
    FMAX_S,
    FCVT_W_S,
    FCVT_WU_S,
    FMV_X_W,
    FEQ_S,
    FLT_S,
    FLE_S,
    FCLASS_S,
    FCVT_S_W,
    FCVT_S_WU,
    FMV_W_X,

    // RV64F Standard Extension
    FCV_TL_S,
    FCV_TLU_S,
    FCV_TS_L,
    FCV_TS_LU,

    // RV32D Standard Extension
    FLD {rd: Register, rs1: Register, imm: u64},
    FSD,
    FMADD_D,
    FMSUB_D,
    FNMSUB_D,
    FNMADD_D,
    FADD_D,
    FSUB_D,
    FMUL_D,
    FDIV_D,
    FSQRT_D,
    FSGNJ_D,
    FSGNJN_D,
    FSGNJX_D,
    FMIN_D,
    FMAX_D,
    S_D,
    D_S,
    FEQ_D,
    FLT_D,
    FLE_D,
    FCLASS_D,
    FCVT_W_D,
    FCVT_WU_D,
    FCVT_D_W,
    FCVT_D_WU,

    // RV64D Standard Extension
    FCVT_L_D,
    FCVT_LU_D,
    FMV_X_D,
    FCVT_D_L,
    FCVT_D_LU,
    FMV_D_X,

}

impl Instruction {
    pub fn decode(instruction: u32) -> Instruction {
        let opcode: usize = (instruction & 0x7F) as usize;
        match &INSTRUCTION_FORMAT_TABLE[opcode] {
            Some(format) => format.decode(instruction),
            None => Instruction::Undefined{
                instruction,
                msg: format!("No instruction format assigned to opcode {:07b}.", opcode)}
        }
    }
}

enum InstructionFormat {
    _R,
    I,
    _S,
    _B,
    U,
    J
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
            }

            // InstructionFormat::U => {
            //     let rd = Register::from((instruction >> 7) & 0b11111);
            //     let imm12 = (instruction >> 12) as u64;
            //     let imm11 = (instruction >> 12) as u64;
            //     let imm1 = (instruction >> 12) as u64;
            //     let imm20 = (instruction >> 12) as u64;
            //
            //     match opcode {
            //
            //         _ => Instruction::Undefined{
            //             instruction,
            //             msg: format!("format: J, opcode: {:07b}", opcode)
            //         }
            //     }
            // }


            _ => Instruction::Undefined{
                instruction,
                msg: format!("Format not implemented")
            }
        }
    }
}

const INSTRUCTION_FORMAT_TABLE: [Option<InstructionFormat>; 128] = [
    /* 0000000 */ None,
    /* 0000001 */ None,
    /* 0000010 */ None,
    /* 0000011 */ Some(InstructionFormat::I),
    /* 0000100 */ None,
    /* 0000101 */ None,
    /* 0000110 */ None,
    /* 0000111 */ Some(InstructionFormat::I),
    /* 0001000 */ None,
    /* 0001001 */ None,
    /* 0001010 */ None,
    /* 0001011 */ None,
    /* 0001100 */ None,
    /* 0001101 */ None,
    /* 0001110 */ None,
    /* 0001111 */ Some(InstructionFormat::I),
    /* 0010000 */ None,
    /* 0010001 */ None,
    /* 0010010 */ None,
    /* 0010011 */ Some(InstructionFormat::I),
    /* 0010100 */ None,
    /* 0010101 */ None,
    /* 0010110 */ None,
    /* 0010111 */ Some(InstructionFormat::U),
    /* 0011000 */ None,
    /* 0011001 */ None,
    /* 0011010 */ None,
    /* 0011011 */ Some(InstructionFormat::I),
    /* 0011100 */ None,
    /* 0011101 */ None,
    /* 0011110 */ None,
    /* 0011111 */ None,
    /* 0100000 */ None,
    /* 0100001 */ None,
    /* 0100010 */ None,
    /* 0100011 */ None,
    /* 0100100 */ None,
    /* 0100101 */ None,
    /* 0100110 */ None,
    /* 0100111 */ None,
    /* 0101000 */ None,
    /* 0101001 */ None,
    /* 0101010 */ None,
    /* 0101011 */ None,
    /* 0101100 */ None,
    /* 0101101 */ None,
    /* 0101110 */ None,
    /* 0101111 */ None,
    /* 0110000 */ None,
    /* 0110001 */ None,
    /* 0110010 */ None,
    /* 0110011 */ None,
    /* 0110100 */ None,
    /* 0110101 */ None,
    /* 0110110 */ None,
    /* 0110111 */ Some(InstructionFormat::U),
    /* 0111000 */ None,
    /* 0111001 */ None,
    /* 0111010 */ None,
    /* 0111011 */ None,
    /* 0111100 */ None,
    /* 0111101 */ None,
    /* 0111110 */ None,
    /* 0111111 */ None,
    /* 1000000 */ None,
    /* 1000001 */ None,
    /* 1000010 */ None,
    /* 1000011 */ None,
    /* 1000100 */ None,
    /* 1000101 */ None,
    /* 1000110 */ None,
    /* 1000111 */ None,
    /* 1001000 */ None,
    /* 1001001 */ None,
    /* 1001010 */ None,
    /* 1001011 */ None,
    /* 1001100 */ None,
    /* 1001101 */ None,
    /* 1001110 */ None,
    /* 1001111 */ None,
    /* 1010000 */ None,
    /* 1010001 */ None,
    /* 1010010 */ None,
    /* 1010011 */ None,
    /* 1010100 */ None,
    /* 1010101 */ None,
    /* 1010110 */ None,
    /* 1010111 */ None,
    /* 1011000 */ None,
    /* 1011001 */ None,
    /* 1011010 */ None,
    /* 1011011 */ None,
    /* 1011100 */ None,
    /* 1011101 */ None,
    /* 1011110 */ None,
    /* 1011111 */ None,
    /* 1100000 */ None,
    /* 1100001 */ None,
    /* 1100010 */ None,
    /* 1100011 */ None,
    /* 1100100 */ None,
    /* 1100101 */ None,
    /* 1100110 */ None,
    /* 1100111 */ Some(InstructionFormat::I),
    /* 1101000 */ None,
    /* 1101001 */ None,
    /* 1101010 */ None,
    /* 1101011 */ None,
    /* 1101100 */ None,
    /* 1101101 */ None,
    /* 1101110 */ None,
    /* 1101111 */ None,
    /* 1110000 */ None,
    /* 1110001 */ None,
    /* 1110010 */ None,
    /* 1110011 */ Some(InstructionFormat::I),
    /* 1110100 */ None,
    /* 1110101 */ None,
    /* 1110110 */ None,
    /* 1110111 */ None,
    /* 1111000 */ None,
    /* 1111001 */ None,
    /* 1111010 */ None,
    /* 1111011 */ None,
    /* 1111100 */ None,
    /* 1111101 */ None,
    /* 1111110 */ None,
    /* 1111111 */ None,
];
