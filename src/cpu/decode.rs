use std::fmt::{Display, Formatter, Debug};
use std::error::Error;

use crate::cpu::instruction::{Instruction, RoundingMode, FloatFormat, InstructionFormat};
use crate::cpu::register::{XRegister, FRegister};
use crate::{bit_concat, bit_slice, sized_bit_extend, sized_bit_slice, bit_extend};


#[derive(Debug)]
pub enum InstructionDecodeError {
    NoInstructionFormatForOpcode { opcode: usize,  instruction: u32 },
    UnknownIInstruction { opcode: usize, rd: XRegister, rs1: XRegister, imm: i64},
    UnknownUInstruction { opcode: usize, rd: XRegister, uimm: u64},
    UnknownRInstruction {
        opcode: usize, rd: XRegister, rs1: XRegister,
        rs2: XRegister, funct3: u32, funct7: u32
    },
    UnknownRFloatInstruction {
        opcode: usize, rd: FRegister, rs1: FRegister, rs2: FRegister,
        rm: RoundingMode, fmt: FloatFormat, func5: u32
    },
    UnknownJInstruction { opcode: usize, imm: i64},
    UnknownSInstruction { opcode: usize, rs1: XRegister, rs2: XRegister, funct3: u32, imm: i64 },
    UnknownBInstruction { opcode: usize, rs1: XRegister, rs2: XRegister, funct3: u32, imm: i64},
}


impl Instruction {
    pub fn decode(instruction: u32) -> Result<Instruction, InstructionDecodeError> {
        let opcode = (instruction & 0x7F) as usize;
        match &INSTRUCTION_FORMAT_TABLE[opcode] {
            Some(format) => format.decode(instruction),
            None => Err(InstructionDecodeError::NoInstructionFormatForOpcode {opcode, instruction})
        }
    }
}


impl InstructionFormat {
    pub fn decode(&self, instruction: u32) -> Result<Instruction, InstructionDecodeError> {
        let opcode: usize = (instruction & 0x7F) as usize;
        match self {
            InstructionFormat::I => {
                let rd_u32 = bit_slice!(instruction, 11, 7);
                let rd = XRegister::from(rd_u32);
                let func3 = bit_slice!(instruction, 14, 12);
                let uimm = bit_slice!(instruction, 19, 15) as u64;
                let rs1 = XRegister::from(uimm as u32);
                let imm = bit_concat!(
                    sized_bit_extend!(bit_slice!(instruction, 31) as u64, 52u64),
                    sized_bit_slice!(instruction as u64, 31, 20)
                ) as i64;

                match (opcode, func3) {
                    (0b0010011, 0b000) => Ok(Instruction::addi{rd, rs1, imm}),
                    (0b0010011, 0b010) => Ok(Instruction::slti{rd, rs1, imm}),
                    (0b0010011, 0b011) => Ok(Instruction::sltiu{rd, rs1, imm: imm as u64}),
                    (0b0010011, 0b100) => Ok(Instruction::xori{rd, rs1, imm: imm as u64}),
                    (0b0010011, 0b110) => Ok(Instruction::ori{rd, rs1, imm: imm as u64}),
                    (0b0010011, 0b111) => Ok(Instruction::andi{rd, rs1, imm: imm as u64}),
                    (0b0010011, 0b001) => Ok(Instruction::slli{rd, rs1, shamt: imm & 0b111111}),
                    (0b0010011, 0b101) => {
                        let shamt = imm & 0b111111;
                        let func6 = imm >> 6;
                        match func6 {
                            0x00 => Ok(Instruction::srli{rd, rs1, shamt}),
                            0x10 => Ok(Instruction::srai{rd, rs1, shamt}),
                            _ => Err(
                                InstructionDecodeError::UnknownIInstruction{opcode, rd, rs1, imm})
                        }
                    },

                    (0b0000011, 0b000) => Ok(Instruction::lb{rd, rs1, imm}),
                    (0b0000011, 0b001) => Ok(Instruction::lh{rd, rs1, imm}),
                    (0b0000011, 0b010) => Ok(Instruction::lw{rd, rs1, imm}),
                    (0b0000011, 0b100) => Ok(Instruction::lbu{rd, rs1, imm}),
                    (0b0000011, 0b101) => Ok(Instruction::lhu{rd, rs1, imm}),
                    (0b0000011, 0b110) => Ok(Instruction::lwu{rd, rs1, imm}),
                    (0b0000011, 0b011) => Ok(Instruction::ld{rd, rs1, imm}),

                    (0b1100111, 0b000) => Ok(Instruction::jalr{rd, rs1, imm}),

                    (0b1110011, 0b000) => {
                        match imm {
                            0 => Ok(Instruction::ecall),
                            1 => Ok(Instruction::ebreak),
                            _ => Err(
                                InstructionDecodeError::UnknownIInstruction{opcode, rd, rs1, imm})
                        }
                    },
                    (0b1110011, 0b001) => Ok(Instruction::csrrw{rd, rs1, imm}),
                    (0b1110011, 0b010) => Ok(Instruction::csrrs{rd, rs1, imm}),
                    (0b1110011, 0b011) => Ok(Instruction::csrrc{rd, rs1, imm}),
                    (0b1110011, 0b101) => Ok(Instruction::csrrwi{rd, uimm, imm}),
                    (0b1110011, 0b110) => Ok(Instruction::csrrsi{rd, uimm, imm}),
                    (0b1110011, 0b111) => Ok(Instruction::csrrci{rd, uimm, imm}),

                    (0b0011011, 0b000) => Ok(Instruction::addiw{rd, rs1, imm}),
                    (0b0011011, 0b001) => Ok(Instruction::slliw{rd, rs1, shamt: imm & 0b11111}),
                    (0b0011011, 0b101) => {
                        let shamt = imm & 0b11111;
                        let func7 = imm >> 5;
                        match func7 {
                            0x00 => Ok(Instruction::srliw{rd, rs1, shamt}),
                            0x20 => Ok(Instruction::sraiw{rd, rs1, shamt}),
                            _ => Err(
                                InstructionDecodeError::UnknownIInstruction{opcode, rd, rs1, imm})
                        }
                    },

                    (0b0001111, 0b001) => Ok(Instruction::fence_i{rd, rs1, imm}),
                    (0b0001111, 0b000) => {
                        let succ = imm as u64 & 0b1111;
                        let pred = (imm as u64 >> 4) & 0b1111;
                        let fm = (imm as u64 >> 8) & 0b1111;
                        match (fm, pred, succ, rs1, rd) {
                            (0b1000, 0b0011, 0b0011, XRegister::x0, XRegister::x0) => Ok(Instruction::fence_tso),
                            (0b0000, 0b0001, 0b0000, XRegister::x0, XRegister::x0) => Ok(Instruction::pause),
                            (fm, pred, succ, rs1, rd) =>
                                Ok(Instruction::fence{rd, rs1, succ, pred, fm})
                        }
                    },

                    (0b0000111, 0b010) => Ok(Instruction::flw{rd: FRegister::from(rd_u32),
                                                           rs1: FRegister::from(uimm), imm}),
                    (0b0000111, 0b011) => Ok(Instruction::fld{rd: FRegister::from(rd_u32),
                                                           rs1: FRegister::from(uimm), imm}),

                    _ => Err(InstructionDecodeError::UnknownIInstruction{opcode, rd, rs1, imm})
                }
            },

            InstructionFormat::U => {
                let rd =  XRegister::from(bit_slice!(instruction, 11, 7));
                let uimm = bit_concat!(
                    sized_bit_extend!(bit_slice!(instruction, 31) as u64, 32),
                    sized_bit_slice!(instruction as u64, 31 , 12),
                    (0b0u64, 12)
                );

                match opcode {
                    0b0110111 => Ok(Instruction::lui{rd, uimm}),
                    0b0010111 => Ok(Instruction::auipc{rd, imm: uimm as i64}),
                    _ => Err(InstructionDecodeError::UnknownUInstruction {opcode, rd, uimm})
                }
            },

            InstructionFormat::R => {
                let ird = bit_slice!(instruction, 11, 7);
                let rd = XRegister::from(ird);
                let funct3 = bit_slice!(instruction, 14, 12);
                let irs1 = bit_slice!(instruction, 19, 15);
                let irs2 = bit_slice!(instruction, 24, 20);
                let rs1 = XRegister::from(irs1);
                let rs2 = XRegister::from(irs2);
                let funct7 = bit_slice!(instruction, 31, 25);

                match opcode {
                    0b0110011 => match (funct3, funct7) {
                        (0x0, 0x00) => Ok(Instruction::add{rd, rs1, rs2}),
                        (0x0, 0x20) => Ok(Instruction::sub{rd, rs1, rs2}),
                        (0x4, 0x0) => Ok(Instruction::xor{rd, rs1, rs2}),
                        (0x6, 0x0) => Ok(Instruction::or{rd, rs1, rs2}),
                        (0x7, 0x0) => Ok(Instruction::and{rd, rs1, rs2}),
                        (0x1, 0x0) => Ok(Instruction::sll{rd, rs1, rs2}),
                        (0x5, 0x00) => Ok(Instruction::srl{rd, rs1, rs2}),
                        (0x5, 0x20) => Ok(Instruction::sra{rd, rs1, rs2}),
                        (0x2, 0x00) => Ok(Instruction::slt{rd, rs1, rs2}),
                        (0x3, 0x00) => Ok(Instruction::sltu{rd, rs1, rs2}),

                        (0x0, 0x01) => Ok(Instruction::mul{rd, rs1, rs2}),
                        (0x1, 0x01) => Ok(Instruction::mulh{rd, rs1, rs2}),
                        (0x2, 0x01) => Ok(Instruction::mulhsu{rd, rs1, rs2}),
                        (0x3, 0x01) => Ok(Instruction::mulhu{rd, rs1, rs2}),
                        (0x4, 0x01) => Ok(Instruction::div{rd, rs1, rs2}),
                        (0x5, 0x01) => Ok(Instruction::divu{rd, rs1, rs2}),
                        (0x6, 0x01) => Ok(Instruction::rem{rd, rs1, rs2}),
                        (0x7, 0x01) => Ok(Instruction::remu{rd, rs1, rs2}),

                        _ => Err(InstructionDecodeError::UnknownRInstruction {
                            opcode, rd, rs1, rs2, funct3, funct7
                        })
                    },

                    0b0111011 => match (funct3, funct7) {
                        (0b000, 0b0000000) => Ok(Instruction::addw{rd, rs1, rs2}),
                        (0b000, 0b0100000) => Ok(Instruction::subw{rd, rs1, rs2}),
                        (0b001, 0b0000000) => Ok(Instruction::sllw{rd, rs1, rs2}),
                        (0b101, 0b0000000) => Ok(Instruction::srlw{rd, rs1, rs2}),
                        (0b101, 0b0100000) => Ok(Instruction::sraw{rd, rs1, rs2}),

                        (0b000, 0b0000001) => Ok(Instruction::mulw{rd, rs1, rs2}),
                        (0b100, 0b0000001) => Ok(Instruction::divw{rd, rs1, rs2}),
                        (0b101, 0b0000001) => Ok(Instruction::divuw{rd, rs1, rs2}),
                        (0b110, 0b0000001) => Ok(Instruction::remw{rd, rs1, rs2}),
                        (0b111, 0b0000001) => Ok(Instruction::remuw{rd, rs1, rs2}),

                        _ => Err(InstructionDecodeError::UnknownRInstruction {
                            opcode, rd, rs1, rs2, funct3, funct7
                        })
                    }

                    0b0101111 => {
                        let rl = (funct7 & 0b1) != 0;
                        let aq = ((funct7 >> 1) & 0b1) != 0;
                        let func5 = funct7 >> 2;

                        match (funct3, func5, irs2) {
                            (0b010, 0b00010, 0x00) => Ok(Instruction::lr_w{rd, rs1, rl, aq}),
                            (0b010, 0b00011, _) => Ok(Instruction::sc_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b00001, _) => Ok(Instruction::amoswap_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b00000, _) => Ok(Instruction::amoadd_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b00100, _) => Ok(Instruction::amoxor_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b01100, _) => Ok(Instruction::amoand_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b01000, _) => Ok(Instruction::amoor_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b10000, _) => Ok(Instruction::amomin_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b10100, _) => Ok(Instruction::amomax_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b11000, _) => Ok(Instruction::amominu_w{rd, rs1, rs2, rl, aq}),
                            (0b010, 0b11100, _) => Ok(Instruction::amomaxu_w{rd, rs1, rs2, rl, aq}),

                            (0b011, 0b00010, 0x00) => Ok(Instruction::lr_d{rd, rs1, rl, aq}),
                            (0b011, 0b00011, _) => Ok(Instruction::sc_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b00001, _) => Ok(Instruction::amoswap_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b00000, _) => Ok(Instruction::amoadd_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b00100, _) => Ok(Instruction::amoxor_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b01100, _) => Ok(Instruction::amoand_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b01000, _) => Ok(Instruction::amoor_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b10000, _) => Ok(Instruction::amomin_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b10100, _) => Ok(Instruction::amomax_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b11000, _) => Ok(Instruction::amominu_d{rd, rs1, rs2, rl, aq}),
                            (0b011, 0b11100, _) => Ok(Instruction::amomaxu_d{rd, rs1, rs2, rl, aq}),

                            _ => Err(InstructionDecodeError::UnknownRInstruction {
                                opcode, rd, rs1, rs2, funct3, funct7
                            })
                        }
                    }

                    // Float instructions
                    0b0100111 | 0b1000011 | 0b1000111 | 0b1001011 | 0b1001111 | 0b1010011 => {
                        let rd = FRegister::from(ird);
                        let rs1 = FRegister::from(irs1);
                        let rs2 = FRegister::from(irs2);
                        let rm = RoundingMode::from(funct3);
                        let fmt = FloatFormat::from(funct7 & 0b11);
                        let func5 = funct7 >> 2;
                        let rs3 = FRegister::from(func5);

                        match (opcode, funct3, irs2, fmt, func5) {
                            (0b0100111, 0b010, _, _, _) => Ok(Instruction::fsw {
                                imm: ((ird as u64)
                                    | ((funct7 << 5) as u64)
                                    | ((funct7 >> 6) as u64 * 0xFFFFFFFFFFFFF000)) as i64,
                                rs1, rs2
                            }),

                            (0b0100111, 0b011, _, _, _) => Ok(Instruction::fsd {
                                imm: ((ird as u64)
                                    | ((funct7 << 5) as u64)
                                    | ((funct7 >> 6) as u64 * 0xFFFFFFFFFFFFF000)) as i64,
                                rs1, rs2
                            }),

                            (0b1000011, _, _, FloatFormat::s, _) => Ok(Instruction::fmadd_s {rd, rm, rs1, rs2, rs3}),
                            (0b1000111, _, _, FloatFormat::s, _) => Ok(Instruction::fmsub_s {rd, rm, rs1, rs2, rs3}),
                            (0b1001011, _, _, FloatFormat::s, _) => Ok(Instruction::fnmsub_s {rd, rm, rs1, rs2, rs3}),
                            (0b1001111, _, _, FloatFormat::s, _) => Ok(Instruction::fnmadd_s {rd, rm, rs1, rs2, rs3}),

                            (0b1010011, _, _, FloatFormat::s, 0b00000) => Ok(Instruction::fadd_s {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::s, 0b00001) => Ok(Instruction::fsub_s {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::s, 0b00010) => Ok(Instruction::fmul_s {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::s, 0b00011) => Ok(Instruction::fdiv_s {rd, rm, rs1, rs2}),
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b01011) => Ok(Instruction::fsqrt_s {rd, rm, rs1}),
                            (0b1010011, 0b000, _, FloatFormat::s, 0b00100) => Ok(Instruction::fsgnj_s {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::s, 0b00100) => Ok(Instruction::fsgnjn_s {rd, rs1, rs2}),
                            (0b1010011, 0b010, _, FloatFormat::s, 0b00100) => Ok(Instruction::fsgnjx_s {rd, rs1, rs2}),
                            (0b1010011, 0b000, _, FloatFormat::s, 0b00101) => Ok(Instruction::fmin_s {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::s, 0b00101) => Ok(Instruction::fmax_s {rd, rs1, rs2}),
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b11000) => Ok(Instruction::fcvt_w_s {rd, rm, rs1}),
                            (0b1010011, _, 0b00001, FloatFormat::s, 0b11000) => Ok(Instruction::fcvt_wu_s {rd, rm, rs1}),
                            (0b1010011, 0b000, 0b00000, FloatFormat::s, 0b11100) => Ok(Instruction::fmv_x_w {rd, rs1}),
                            (0b1010011, 0b010, _, FloatFormat::s, 0b10100) => Ok(Instruction::feq_s {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::s, 0b10100) => Ok(Instruction::flt_s {rd, rs1, rs2}),
                            (0b1010011, 0b000, _, FloatFormat::s, 0b10100) => Ok(Instruction::fle_s {rd, rs1, rs2}),
                            (0b1010011, 0b001, 0b00000, FloatFormat::s, 0b11100) => Ok(Instruction::fclass_s {rd, rs1}),
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b11010) => Ok(Instruction::fcvt_s_w {rd, rm, rs1}),
                            (0b1010011, _, 0b00001, FloatFormat::s, 0b11010) => Ok(Instruction::fcvt_s_wu {rd, rm, rs1}),
                            (0b1010011, 0b000, 0b00000, FloatFormat::s, 0b11110) => Ok(Instruction::fmv_w_x {rd, rs1}),
                            (0b1010011, _, 0b00010, FloatFormat::s, 0b11000) => Ok(Instruction::fcv_tl_s {rd, rm, rs1}),
                            (0b1010011, _, 0b00011, FloatFormat::s, 0b11000) => Ok(Instruction::fcv_tlu_s {rd, rm, rs1}),
                            (0b1010011, _, 0b00010, FloatFormat::s, 0b11010) => Ok(Instruction::fcv_ts_l {rd, rm, rs1}),
                            (0b1010011, _, 0b00011, FloatFormat::s, 0b11010) => Ok(Instruction::fcv_ts_lu {rd, rm, rs1}),

                            (0b1000011, _, _, FloatFormat::d, _) => Ok(Instruction::fmadd_d {rd, rm, rs1, rs2, rs3}),
                            (0b1000111, _, _, FloatFormat::d, _) => Ok(Instruction::fmsub_d {rd, rm, rs1, rs2, rs3}),
                            (0b1001011, _, _, FloatFormat::d, _) => Ok(Instruction::fnmsub_d {rd, rm, rs1, rs2, rs3}),
                            (0b1001111, _, _, FloatFormat::d, _) => Ok(Instruction::fnmadd_d {rd, rm, rs1, rs2, rs3}),
                            (0b1010011, _, _, FloatFormat::d, 0b00000) => Ok(Instruction::fadd_d {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::d, 0b00001) => Ok(Instruction::fsub_d {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::d, 0b00010) => Ok(Instruction::fmul_d {rd, rm, rs1, rs2}),
                            (0b1010011, _, _, FloatFormat::d, 0b00011) => Ok(Instruction::fdiv_d {rd, rm, rs1, rs2}),
                            (0b1010011, _, 0b00000, FloatFormat::d, 0b01011) => Ok(Instruction::fsqrt_d {rd, rm, rs1}),
                            (0b1010011, 0b000, _, FloatFormat::d, 0b00100) => Ok(Instruction::fsgnj_d {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::d, 0b00100) => Ok(Instruction::fsgnjn_d {rd, rs1, rs2}),
                            (0b1010011, 0b010, _, FloatFormat::d, 0b00100) => Ok(Instruction::fsgnjx_d {rd, rs1, rs2}),
                            (0b1010011, 0b000, _, FloatFormat::d, 0b00101) => Ok(Instruction::fmin_d {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::d, 0b00101) => Ok(Instruction::fmax_d {rd, rs1, rs2}),
                            (0b1010011, _, 0b00001, FloatFormat::d, 0b01000) => Ok(Instruction::fcvt_s_d {rd, rm, rs1}),
                            (0b1010011, _, 0b00000, FloatFormat::d, 0b01000) => Ok(Instruction::fcvt_d_s {rd, rm, rs1}),
                            (0b1010011, 0b010, _, FloatFormat::d, 0b10100) => Ok(Instruction::feq_d {rd, rs1, rs2}),
                            (0b1010011, 0b001, _, FloatFormat::d, 0b10100) => Ok(Instruction::flt_d {rd, rs1, rs2}),
                            (0b1010011, 0b000, _, FloatFormat::d, 0b10100) => Ok(Instruction::fle_d {rd, rs1, rs2}),
                            (0b1010011, 0b001, 0b00000, FloatFormat::d, 0b11100) => Ok(Instruction::fclass_d {rd, rs1}),
                            (0b1010011, _, 0b00000, FloatFormat::d, 0b11000) => Ok(Instruction::fcvt_w_d {rd, rm, rs1}),
                            (0b1010011, _, 0b00001, FloatFormat::d, 0b11000) => Ok(Instruction::fcvt_wu_d {rd, rm, rs1}),
                            (0b1010011, _, 0b00000, FloatFormat::d, 0b11010) => Ok(Instruction::fcvt_d_w {rd, rm, rs1}),
                            (0b1010011, _, 0b00001, FloatFormat::d, 0b11010) => Ok(Instruction::fcvt_d_wu {rd, rm, rs1}),
                            (0b1010011, _, 0b00010, FloatFormat::d, 0b11000) => Ok(Instruction::fcvt_l_d {rd, rm, rs1}),
                            (0b1010011, _, 0b00011, FloatFormat::d, 0b11000) => Ok(Instruction::fcvt_lu_d {rd, rm, rs1}),
                            (0b1010011, 0b000, 0b00000, FloatFormat::d, 0b11100) => Ok(Instruction::fmv_x_d {rd, rs1}),
                            (0b1010011, _, 0b00010, FloatFormat::d, 0b11010) => Ok(Instruction::fcvt_d_l {rd, rm, rs1}),
                            (0b1010011, _, 0b00011, FloatFormat::d, 0b11010) => Ok(Instruction::fcvt_d_lu {rd, rm, rs1}),
                            (0b1010011, 0b000, 0b00000, FloatFormat::d, 0b11110) => Ok(Instruction::fmv_d_x {rd, rs1}),


                            _ => Err(InstructionDecodeError::UnknownRFloatInstruction {
                                opcode, rd, rs1, rs2, rm, fmt, func5
                            })
                        }
                    }

                    _ => Err(InstructionDecodeError::UnknownRInstruction {
                        opcode, rd, rs1, rs2, funct3, funct7
                    })
                }
            },

            InstructionFormat::J => {
                let rd = XRegister::from(bit_slice!(instruction, 11, 7));
                let imm = bit_concat!(
                    sized_bit_extend!(bit_slice!(instruction, 31) as u64, 64),
                    sized_bit_slice!(instruction as u64, 18, 10),
                    sized_bit_slice!(instruction as u64, 19),
                    sized_bit_slice!(instruction as u64, 30, 20)
                ) as i64;

                match opcode {
                    1101111 => Ok(Instruction::jal{rd, imm}),
                    _ => Err(InstructionDecodeError::UnknownJInstruction{ opcode, imm })
                }
            }

            InstructionFormat::S => {

                let funct3 = bit_slice!(instruction, 14, 12);
                let rs1 = XRegister::from(bit_slice!(instruction, 19, 15));
                let rs2 = XRegister::from(bit_slice!(instruction, 24, 20));
                let imm = bit_concat!(
                    sized_bit_extend!(bit_slice!(instruction, 31) as u64, 52),
                    sized_bit_slice!(instruction as u64, 31, 25),
                    sized_bit_slice!(instruction as u64, 11, 7)
                ) as i64;

                match (opcode, funct3) {
                        (0b0100011, 0b000) => Ok(Instruction::sb{rs1, rs2, imm}),
                        (0b0100011, 0b001) => Ok(Instruction::sh{rs1, rs2, imm}),
                        (0b0100011, 0b010) => Ok(Instruction::sw{rs1, rs2, imm}),
                        (0b0100011, 0b011) => Ok(Instruction::sd{rs1, rs2, imm}),

                    _ => Err(InstructionDecodeError::UnknownSInstruction{
                        opcode, rs1, rs2, funct3, imm
                    })
                }
            }

            InstructionFormat::B => {
                let funct3 = bit_slice!(instruction, 14, 12);
                let rs1 = XRegister::from(bit_slice!(instruction, 19, 15));
                let rs2 = XRegister::from(bit_slice!(instruction, 24, 20));
                let imm = bit_concat!(
                    sized_bit_extend!(bit_slice!(instruction, 31) as u64, 53),
                    sized_bit_slice!(instruction as u64, 7),
                    sized_bit_slice!(instruction as u64, 30, 25),
                    sized_bit_slice!(instruction as u64, 11, 8),
                    (0, 1)
                ) as i64;

                match (opcode, funct3) {
                    (0b1100011, 0b000) => Ok(Instruction::beq{rs1, rs2, imm}),
                    (0b1100011, 0b001) => Ok(Instruction::bne{rs1, rs2, imm}),
                    (0b1100011, 0b100) => Ok(Instruction::blt{rs1, rs2, imm}),
                    (0b1100011, 0b101) => Ok(Instruction::bge{rs1, rs2, imm}),
                    (0b1100011, 0b110) => Ok(Instruction::bltu{rs1, rs2, imm}),
                    (0b1100011, 0b111) => Ok(Instruction::bgeu{rs1, rs2, imm}),
                    _ => Err(InstructionDecodeError::UnknownBInstruction{
                        opcode, rs1, rs2, funct3, imm
                    })
                }
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
    /* 0b0100011 */ Some(InstructionFormat::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ Some(InstructionFormat::R),
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ Some(InstructionFormat::R),
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(InstructionFormat::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(InstructionFormat::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ Some(InstructionFormat::R),
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ Some(InstructionFormat::R),
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ Some(InstructionFormat::R),
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ Some(InstructionFormat::R),
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ Some(InstructionFormat::R),
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ Some(InstructionFormat::R),
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
    /* 0b1100011 */ Some(InstructionFormat::B),
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
    /* 0b1101111 */ Some(InstructionFormat::J),
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

impl Display for InstructionDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InstructionDecodeError { }
