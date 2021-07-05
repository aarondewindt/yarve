use crate::cpu::register::Register;
use crate::cpu::register::FloatRegister;

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum RoundingMode {
    Invalid {rm: u32},
    rne, // Round to Nearest, ties to Even
    rtz, // Round towards Zero
    rdn, // Round Down (towards −∞)
    rup, // Round Up (towards +∞)
    rmm, // Round to Nearest, ties to Max Magnitude
    r#dyn, // In instruction’s rm field, selects dynamic rounding mode;
         // In Rounding Mode register, reserved.
}

impl From<u32> for RoundingMode {
    fn from(value: u32) -> RoundingMode {
        if let Some(rounding_mode) = ROUNDING_MODE_TABLE[value as usize] {
            rounding_mode
        } else {
            RoundingMode::Invalid{rm: value}
        }
    }
}

const ROUNDING_MODE_TABLE: [Option<RoundingMode>; 8] = [
    /* 0b000 */ Some(RoundingMode::rne),
    /* 0b001 */ Some(RoundingMode::rtz),
    /* 0b010 */ Some(RoundingMode::rdn),
    /* 0b011 */ Some(RoundingMode::rup),
    /* 0b100 */ Some(RoundingMode::rmm),
    /* 0b101 */ None,
    /* 0b110 */ None,
    /* 0b111 */ Some(RoundingMode::r#dyn),
];


#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FloatFormat {
    s,
    d,
    h,
    q,
}

impl From<u32> for FloatFormat {
    fn from(value: u32) -> FloatFormat {
        FLOAT_FORMAT_TABLE[value as usize]
    }
}

const FLOAT_FORMAT_TABLE: [FloatFormat; 4] = [
    /* 0b00 */ FloatFormat::s,
    /* 0b01 */ FloatFormat::d,
    /* 0b10 */ FloatFormat::h,
    /* 0b11 */ FloatFormat::q,
];

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    Undefined {instruction: u32, msg: String},

    // RV32I Base Instruction Set
    // R: 0110011
    add {rd: Register, rs1: Register, rs2: Register},
    sub {rd: Register, rs1: Register, rs2: Register},
    xor {rd: Register, rs1: Register, rs2: Register},
    or {rd: Register, rs1: Register, rs2: Register},
    and {rd: Register, rs1: Register, rs2: Register},
    sll {rd: Register, rs1: Register, rs2: Register},
    srl {rd: Register, rs1: Register, rs2: Register},
    sra {rd: Register, rs1: Register, rs2: Register},
    slt {rd: Register, rs1: Register, rs2: Register},
    sltu {rd: Register, rs1: Register, rs2: Register},

    // I: 0010011
    addi {rd: Register, rs1: Register, imm: u64},
    xori {rd: Register, rs1: Register, imm: u64},
    ori {rd: Register, rs1: Register, imm: u64},
    andi {rd: Register, rs1: Register, imm: u64},
    slli {rd: Register, rs1: Register, shamt: u64},
    srli {rd: Register, rs1: Register, shamt: u64},
    srai {rd: Register, rs1: Register, shamt: u64},
    slti {rd: Register, rs1: Register, imm: u64},
    sltiu {rd: Register, rs1: Register, imm: u64},

    // I: 0000011
    lb {rd: Register, rs1: Register, imm: u64},
    lh {rd: Register, rs1: Register, imm: u64},
    lw {rd: Register, rs1: Register, imm: u64},
    lbu {rd: Register, rs1: Register, imm: u64},
    lhu {rd: Register, rs1: Register, imm: u64},

    // S: 0100011
    sb {rs1: Register, rs2: Register, imm: u64},
    sh {rs1: Register, rs2: Register, imm: u64},
    sw {rs1: Register, rs2: Register, imm: u64},

    // B: 1100011
    beq {rs1: Register, rs2: Register, imm: u64},
    bne {rs1: Register, rs2: Register, imm: u64},
    blt {rs1: Register, rs2: Register, imm: u64},
    bge {rs1: Register, rs2: Register, imm: u64},
    bltu {rs1: Register, rs2: Register, imm: u64},
    bgeu {rs1: Register, rs2: Register, imm: u64},

    jal {rd: Register, imm: u64},  // J: 1101111
    jalr {rd: Register, rs1: Register, imm: u64},  // I: 1100111

    lui {rd: Register, imm: u64},  // U: 0110111
    auipc {rd: Register, imm: u64},  // U: 0010111

    // I: 1110011
    ecall,
    ebreak,

    // ?: 0001111
    fence {rd: Register, rs1: Register, succ: u64, pred: u64, fm: u64},
    fence_tso,
    pause,

    // RV64I Base Instruction Set
    // I: 0000011
    lwu {rd: Register, rs1: Register, imm: u64},
    ld {rd: Register, rs1: Register, imm: u64},

    // R: 0100011
    sd {rs1: Register, rs2: Register, imm: u64},

    // I: 0011011
    addiw {rd: Register, rs1: Register, imm: u64},

    // R: 0011011
    slliw {rd: Register, rs1: Register, shamt: u64},
    srliw {rd: Register, rs1: Register, shamt: u64},
    sraiw {rd: Register, rs1: Register, shamt: u64},

    // R: 0111011
    addw {rd: Register, rs1: Register, rs2: Register},
    subw {rd: Register, rs1: Register, rs2: Register},
    sllw {rd: Register, rs1: Register, rs2: Register},
    srlw {rd: Register, rs1: Register, rs2: Register},
    sraw {rd: Register, rs1: Register, rs2: Register},

    // RV32/RV64 Zifencei Standard Extension
    fence_i {rd: Register, rs1: Register, imm: u64},

    // RV32/RV64 Zicsr Standard Extension
    csrrw {rd: Register, rs1: Register, imm: u64},
    csrrs {rd: Register, rs1: Register, imm: u64},
    csrrc {rd: Register, rs1: Register, imm: u64},
    csrrwi {rd: Register, uimm: u64, imm: u64},
    csrrsi {rd: Register, uimm: u64, imm: u64},
    csrrci {rd: Register, uimm: u64, imm: u64},

    // RV32M Standard Extension
    // R: 0110011
    mul {rd: Register, rs1: Register, rs2: Register},
    mulh {rd: Register, rs1: Register, rs2: Register},
    mulhsu {rd: Register, rs1: Register, rs2: Register},
    mulhu {rd: Register, rs1: Register, rs2: Register},
    div {rd: Register, rs1: Register, rs2: Register},
    divu {rd: Register, rs1: Register, rs2: Register},
    rem {rd: Register, rs1: Register, rs2: Register},
    remu {rd: Register, rs1: Register, rs2: Register},

    // RV64M Standard Extension
    mulw {rd: Register, rs1: Register, rs2: Register},
    divw {rd: Register, rs1: Register, rs2: Register},
    divuw {rd: Register, rs1: Register, rs2: Register},
    remw {rd: Register, rs1: Register, rs2: Register},
    remuw {rd: Register, rs1: Register, rs2: Register},

    // RV32A Standard Extension
    // R: 0101111
    lr_w {rd: Register, rs1: Register, rl: bool, aq: bool},
    sc_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoswap_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoadd_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoxor_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoand_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoor_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomin_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomax_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amominu_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomaxu_w {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},

    // RV64A Standard Extension
    lr_d {rd: Register, rs1: Register, rl: bool, aq: bool},
    sc_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoswap_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoadd_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoxor_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoand_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amoor_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomin_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomax_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amominu_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},
    amomaxu_d {rd: Register, rs1: Register, rs2: Register, rl: bool, aq: bool},

    // RV32F Standard Extension
    // I: 0000111
    flw {rd: FloatRegister, rs1: FloatRegister, imm: u64},

    // R: 0100111
    fsw {imm: u64, rs1: FloatRegister, rs2: FloatRegister},

    // R4: 1000011
    fmadd_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},

    // R4: 1000111
    fmsub_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},

    // R4: 1001011
    fnmsub_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},

    // R4: 1001111
    fnmadd_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},

    // R: 1010011
    fadd_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fsub_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fmul_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fdiv_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fsqrt_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fsgnj_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fsgnjn_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fsgnjx_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fmin_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fmax_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fcvt_w_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_wu_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fmv_x_w {rd: FloatRegister, rs1: FloatRegister},
    feq_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    flt_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fle_s {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fclass_s {rd: FloatRegister, rs1: FloatRegister},
    fcvt_s_w {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_s_wu {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fmv_w_x {rd: FloatRegister, rs1: FloatRegister},

    // RV64F Standard Extension
    // R: 1010011
    fcv_tl_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcv_tlu_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcv_ts_l {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcv_ts_lu {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},

    // RV32D Standard Extension
    fld {rd: FloatRegister, rs1: FloatRegister, imm: u64},
    _fsd,
    _fmadd_d,
    _fmsub_d,
    _fnmsub_d,
    _fnmadd_d,
    _fadd_d,
    _fsub_d,
    _fmul_d,
    _fdiv_d,
    _fsqrt_d,
    _fsgnj_d,
    _fsgnjn_d,
    _fsgnjx_d,
    _fmin_d,
    _fmax_d,
    _s_d,
    _d_s,
    _feq_d,
    _flt_d,
    _fle_d,
    _fclass_d,
    _fcvt_w_d,
    _fcvt_wu_d,
    _fcvt_d_w,
    _fcvt_d_wu,

    // RV64D Standard Extension
    _fcvt_l_d,
    _fcvt_lu_d,
    _fmv_x_d,
    _fcvt_d_l,
    _fcvt_d_lu,
    _fmv_d_x,

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
    S,
    B,
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
                    0xFFFFFFFFFFFFF000u64 * (instruction as u64 >> 31)
                    | (instruction as u64 >> 20);

                match opcode {
                    0b0010011 => {
                        match func3 {
                            0b000 => Instruction::addi{rd, rs1, imm},
                            0b010 => Instruction::slti{rd, rs1, imm},
                            0b011 => Instruction::sltiu{rd, rs1, imm},
                            0b100 => Instruction::xori{rd, rs1, imm},
                            0b110 => Instruction::ori{rd, rs1, imm},
                            0b111 => Instruction::andi{rd, rs1, imm},
                            0b001 => Instruction::slli{rd, rs1, shamt: imm & 0b111111},
                            0b101 => {
                                let shamt = imm & 0b111111;
                                let func6 = imm >> 6;
                                match func6 {
                                    0x00 => Instruction::srli{rd, rs1, shamt},
                                    0x10 => Instruction::srai{rd, rs1, shamt},
                                    _ => Instruction::Undefined{
                                        instruction,
                                        msg: format!("format: I, opcode: {:07b}, func3: {:b}, \
                                                  func6: {:b}", opcode, func3, func6)
                                    }
                                }
                            },

                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0000011 => {
                        match func3 {
                            0b000 => Instruction::lb{rd, rs1, imm},
                            0b001 => Instruction::lh{rd, rs1, imm},
                            0b010 => Instruction::lw{rd, rs1, imm},
                            0b100 => Instruction::lbu{rd, rs1, imm},
                            0b101 => Instruction::lhu{rd, rs1, imm},
                            0b110 => Instruction::lwu{rd, rs1, imm},
                            0b011 => Instruction::ld{rd, rs1, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b1100111 => {
                        match func3 {
                            0b000 => Instruction::jalr{rd, rs1, imm},
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
                                    0 => Instruction::ecall,
                                    1 => Instruction::ebreak,
                                    _ => Instruction::Undefined{
                                        instruction,
                                        msg: format!(
                                            "format: I, opcode: {:07b}, func3: {:b}, imm {:b}",
                                            opcode, func3, imm)
                                    }
                                }
                            },
                            0b001 => Instruction::csrrw{rd, rs1, imm},
                            0b010 => Instruction::csrrs{rd, rs1, imm},
                            0b011 => Instruction::csrrc{rd, rs1, imm},
                            0b101 => Instruction::csrrwi{rd, uimm, imm},
                            0b110 => Instruction::csrrsi{rd, uimm, imm},
                            0b111 => Instruction::csrrci{rd, uimm, imm},
                            _ => Instruction::Undefined {
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0011011 => {
                        match func3 {
                            0b000 => Instruction::addiw{rd, rs1, imm},
                            0b001 => Instruction::slliw{rd, rs1, shamt: imm & 0b11111},
                            0b101 => {
                                let shamt = imm & 0b11111;
                                let func7 = imm >> 5;
                                match func7 {
                                    0x00 => Instruction::srliw{rd, rs1, shamt},
                                    0x20 => Instruction::sraiw{rd, rs1, shamt},
                                    _ => Instruction::Undefined{
                                        instruction,
                                        msg: format!("format: I, opcode: {:07b}, func3: {:b}, \
                                                  func7: {:b}", opcode, func3, func7)
                                    }
                                }
                            },
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0001111 => {
                        match func3 {
                            0b001 => Instruction::fence_i{rd, rs1, imm},
                            0b000 => {
                                let succ = imm & 0b1111;
                                let pred = (imm >> 4) & 0b1111;
                                let fm = (imm >> 8) & 0b1111;
                                match (fm, pred, succ, rs1, rd) {
                                    (0b1000, 0b0011, 0b0011, Register::x0, Register::x0) => Instruction::fence_tso,
                                    (0b0000, 0b0001, 0b0000, Register::x0, Register::x0) => Instruction::pause,
                                    (fm, pred, succ, rs1, rd) =>
                                        Instruction::fence{rd, rs1, succ, pred, fm}
                                }
                            },
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: I, opcode: {:07b}, func3: {:b}", opcode, func3)
                            }
                        }
                    },

                    0b0000111 => {
                        match func3 {
                            0b010 => Instruction::flw{rd: FloatRegister::from(rd),
                                                      rs1: FloatRegister::from(rs1), imm},
                            0b011 => Instruction::fld{rd: FloatRegister::from(rd),
                                                      rs1: FloatRegister::from(rs1), imm},
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
                    0b0110111 => Instruction::lui{rd, imm},
                    0b0010111 => Instruction::auipc{rd, imm},

                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: U, opcode: {:07b}", opcode)
                    }
                }
            },

            InstructionFormat::R => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let func3 = (instruction >> 12) & 0b111;
                let irs1 = (instruction >> 15) & 0b11111;
                let irs2 = (instruction >> 20) & 0b11111;
                let rs1 = Register::from(irs1);
                let rs2 = Register::from(irs2);
                let func7 = instruction >> 25;

                match opcode {
                    0b0110011 => match (func3, func7) {
                        (0x0, 0x00) => {Instruction::add{rd, rs1, rs2}},
                        (0x0, 0x20) => {Instruction::sub{rd, rs1, rs2}},
                        (0x4, 0x0, ) => {Instruction::xor{rd, rs1, rs2}},
                        (0x6, 0x0, ) => {Instruction::or{rd, rs1, rs2}},
                        (0x7, 0x0, ) => {Instruction::and{rd, rs1, rs2}},
                        (0x1, 0x0, ) => {Instruction::sll{rd, rs1, rs2}},
                        (0x5, 0x00) => {Instruction::srl{rd, rs1, rs2}},
                        (0x5, 0x20) => {Instruction::sra{rd, rs1, rs2}},
                        (0x2, 0x00) => {Instruction::slt{rd, rs1, rs2}},
                        (0x3, 0x00) => {Instruction::sltu{rd, rs1, rs2}},

                        (0x0, 0x01) => Instruction::mul{rd, rs1, rs2},
                        (0x1, 0x01) => Instruction::mulh{rd, rs1, rs2},
                        (0x2, 0x01) => Instruction::mulhsu{rd, rs1, rs2},
                        (0x3, 0x01) => Instruction::mulhu{rd, rs1, rs2},
                        (0x4, 0x01) => Instruction::div{rd, rs1, rs2},
                        (0x5, 0x01) => Instruction::divu{rd, rs1, rs2},
                        (0x6, 0x01) => Instruction::rem{rd, rs1, rs2},
                        (0x7, 0x01) => Instruction::remu{rd, rs1, rs2},

                        _ => Instruction::Undefined{
                            instruction,
                            msg: format!("format: R, opcode: {:07b}, \
                                          func3: {:b}, func7: {:b}", opcode, func3, func7)
                        }
                    },

                    0b0111011 => match (func3, func7) {
                        (0b000, 0b0000000) => Instruction::addw{rd, rs1, rs2},
                        (0b000, 0b0100000) => Instruction::subw{rd, rs1, rs2},
                        (0b001, 0b0000000) => Instruction::sllw{rd, rs1, rs2},
                        (0b101, 0b0000000) => Instruction::srlw{rd, rs1, rs2},
                        (0b101, 0b0100000) => Instruction::sraw{rd, rs1, rs2},

                        (0b000, 0b0000001) => Instruction::mulw{rd, rs1, rs2},
                        (0b100, 0b0000001) => Instruction::divw{rd, rs1, rs2},
                        (0b101, 0b0000001) => Instruction::divuw{rd, rs1, rs2},
                        (0b110, 0b0000001) => Instruction::remw{rd, rs1, rs2},
                        (0b111, 0b0000001) => Instruction::remuw{rd, rs1, rs2},

                        _ => Instruction::Undefined{
                            instruction,
                            msg: format!("format: R, opcode: {:07b}, \
                                          func3: {:b}, func7: {:b}", opcode, func3, func7)
                        }
                    }

                    0b0101111 => {
                        let rl = (func7 & 0b1) != 0;
                        let aq = ((func7 >> 1) & 0b1) != 0;
                        let func5 = func7 >> 2;

                        match (func3, func5, irs2) {
                            (0b010, 0b00010, 0x00) => Instruction::lr_w{rd, rs1, rl, aq},
                            (0b010, 0b00011, _) => Instruction::sc_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b00001, _) => Instruction::amoswap_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b00000, _) => Instruction::amoadd_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b00100, _) => Instruction::amoxor_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b01100, _) => Instruction::amoand_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b01000, _) => Instruction::amoor_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b10000, _) => Instruction::amomin_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b10100, _) => Instruction::amomax_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b11000, _) => Instruction::amominu_w{rd, rs1, rs2, rl, aq},
                            (0b010, 0b11100, _) => Instruction::amomaxu_w{rd, rs1, rs2, rl, aq},

                            (0b011, 0b00010, 0x00) => Instruction::lr_d{rd, rs1, rl, aq},
                            (0b011, 0b00011, _) => Instruction::sc_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b00001, _) => Instruction::amoswap_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b00000, _) => Instruction::amoadd_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b00100, _) => Instruction::amoxor_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b01100, _) => Instruction::amoand_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b01000, _) => Instruction::amoor_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b10000, _) => Instruction::amomin_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b10100, _) => Instruction::amomax_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b11000, _) => Instruction::amominu_d{rd, rs1, rs2, rl, aq},
                            (0b011, 0b11100, _) => Instruction::amomaxu_d{rd, rs1, rs2, rl, aq},

                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: R, opcode: {:07b}, \
                                          func3: {:b}, func7: {:b}", opcode, func3, func7)
                            }
                        }
                    }

                    // Float instructions
                    0b0100111 | 0b1000011 | 0b1000111 | 0b1001011 | 0b1001111 | 0b1010011 => {
                        let rd = FloatRegister::from(rd);
                        let rs1 = FloatRegister::from(rs1);
                        let rs2 = FloatRegister::from(rs2);
                        let rm = RoundingMode::from(func3);
                        let fmt = FloatFormat::from(func7 & 0b11);
                        let func5 = func7 >> 2;
                        let rs3 = FloatRegister::from(func5);

                        match (opcode, func3, irs2, fmt, func5) {
                            (0b0100111, 0b010, _, _, _) => Instruction::fsw {
                                imm: u64::from(rd)
                                    | ((func7 << 5) as u64)
                                    | ((func7 >> 6) as u64 * 0xFFFFFFFFFFFFF000),
                                rs1, rs2
                            },

                            (0b1000011, _, _, FloatFormat::s, _) => Instruction::fmadd_s {rd, rm, rs1, rs2, rs3},
                            (0b1000111, _, _, FloatFormat::s, _) => Instruction::fmsub_s {rd, rm, rs1, rs2, rs3},
                            (0b1001011, _, _, FloatFormat::s, _) => Instruction::fnmsub_s {rd, rm, rs1, rs2, rs3},
                            (0b1001111, _, _, FloatFormat::s, _) => Instruction::fnmadd_s {rd, rm, rs1, rs2, rs3},
                            (0b1010011, _, _, FloatFormat::s, 0b00000) => Instruction::fadd_s {rd, rm, rs1, rs2},
                            (0b1010011, _, _, FloatFormat::s, 0b00001) => Instruction::fsub_s {rd, rm, rs1, rs2},
                            (0b1010011, _, _, FloatFormat::s, 0b00010) => Instruction::fmul_s {rd, rm, rs1, rs2},
                            (0b1010011, _, _, FloatFormat::s, 0b00011) => Instruction::fdiv_s {rd, rm, rs1, rs2},
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b01011) => Instruction::fsqrt_s {rd, rm, rs1},
                            (0b1010011, 0b000, _, FloatFormat::s, 0b00100) => Instruction::fsgnj_s {rd, rs1, rs2},
                            (0b1010011, 0b001, _, FloatFormat::s, 0b00100) => Instruction::fsgnjn_s {rd, rs1, rs2},
                            (0b1010011, 0b010, _, FloatFormat::s, 0b00100) => Instruction::fsgnjx_s {rd, rs1, rs2},
                            (0b1010011, 0b000, _, FloatFormat::s, 0b00101) => Instruction::fmin_s {rd, rs1, rs2},
                            (0b1010011, 0b001, _, FloatFormat::s, 0b00101) => Instruction::fmax_s {rd, rs1, rs2},
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b11000) => Instruction::fcvt_w_s {rd, rm, rs1},
                            (0b1010011, _, 0b00001, FloatFormat::s, 0b11000) => Instruction::fcvt_wu_s {rd, rm, rs1},
                            (0b1010011, 0b000, 0b00000, FloatFormat::s, 0b11100) => Instruction::fmv_x_w {rd, rs1},
                            (0b1010011, 0b010, _, FloatFormat::s, 0b10100) => Instruction::feq_s {rd, rs1, rs2},
                            (0b1010011, 0b001, _, FloatFormat::s, 0b10100) => Instruction::flt_s {rd, rs1, rs2},
                            (0b1010011, 0b000, _, FloatFormat::s, 0b10100) => Instruction::fle_s {rd, rs1, rs2},
                            (0b1010011, 0b001, 0b00000, FloatFormat::s, 0b11100) => Instruction::fclass_s {rd, rs1},
                            (0b1010011, _, 0b00000, FloatFormat::s, 0b11010) => Instruction::fcvt_s_w {rd, rm, rs1},
                            (0b1010011, _, 0b00001, FloatFormat::s, 0b11010) => Instruction::fcvt_s_wu {rd, rm, rs1},
                            (0b1010011, 0b000, 0b00000, FloatFormat::s, 0b11110) => Instruction::fmv_w_x {rd, rs1},
                            (0b1010011, _, 0b00010, FloatFormat::s, 0b11000) => Instruction::fcv_tl_s {rd, rm, rs1},
                            (0b1010011, _, 0b00011, FloatFormat::s, 0b11000) => Instruction::fcv_tlu_s {rd, rm, rs1},
                            (0b1010011, _, 0b00010, FloatFormat::s, 0b11010) => Instruction::fcv_ts_l {rd, rm, rs1},
                            (0b1010011, _, 0b00011, FloatFormat::s, 0b11010) => Instruction::fcv_ts_lu {rd, rm, rs1},


                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: R, float instr, opcode: {:07b}, \
                                              func3: {:03b}, fmt: {:?}, func5: {:03b}",
                                             opcode, func3, fmt, func5)
                            }
                        }
                    }

                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: R, opcode: {:07b}", opcode)
                    }
                }
            },

            InstructionFormat::J => {
                let rd = Register::from((instruction >> 7) & 0b11111);
                let imm1912 = ((instruction >> 12) & 0b11111111) as u64;
                let imm11 = ((instruction >> 20) & 0b1) as u64;
                let imm101 = ((instruction >> 21) & 0b1111111111) as u64;
                let imm20 = (instruction >> 31) as u64;

                let imm = 0xFFFFFFFFFFF00000 * imm20 |
                    (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);

                match opcode {
                    1101111 => {Instruction::jal{rd, imm}},
                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: J, opcode: {:07b}", opcode)
                    }
                }
            }

            InstructionFormat::S => {
                let imm40 = ((instruction >> 7) & 0b11111) as u64;
                let func3 = (instruction >> 12) & 0b111;
                let rs1 = Register::from((instruction >> 15) & 0b11111);
                let rs2 = Register::from((instruction >> 20) & 0b11111);
                let imm115 = (instruction >> 25) as u64;
                let imm = 0xFFFFFFFFFFFFF000 * (instruction as u64 >> 31)
                    | (imm115 << 5) | imm40;

                match opcode {
                    0b0100011 => {
                        match func3 {
                            0b000 => Instruction::sb{rs1, rs2, imm},
                            0b001 => Instruction::sh{rs1, rs2, imm},
                            0b010 => Instruction::sw{rs1, rs2, imm},
                            0b011 => Instruction::sd{rs1, rs2, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: S, opcode: {:07b}, func3: {:03b}", opcode, func3)
                            }
                        }
                    }
                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: S, opcode: {:07b}", opcode)
                    }
                }
            }

            InstructionFormat::B => {
                let imm11 = ((instruction >> 7) & 0b1) as u64;
                let imm41 = ((instruction >> 8) & 0b1111) as u64;
                let func3 = (instruction >> 12) & 0b111;
                let rs1 = Register::from((instruction >> 15) & 0b11111);
                let rs2 = Register::from((instruction >> 20) & 0b11111);
                let imm105 = ((instruction >> 25) & 0b111111) as u64;
                let imm12 = (instruction >> 31) as u64;

                let imm = 0xFFFFFFFFFFFFF000 * imm12 |
                    (imm11 << 11) | (imm105 << 5) | (imm41 << 1);

                match opcode {
                    0b1100011 => {
                        match func3 {
                            0b000 => Instruction::beq{rs1, rs2, imm},
                            0b001 => Instruction::bne{rs1, rs2, imm},
                            0b100 => Instruction::blt{rs1, rs2, imm},
                            0b101 => Instruction::bge{rs1, rs2, imm},
                            0b110 => Instruction::bltu{rs1, rs2, imm},
                            0b111 => Instruction::bgeu{rs1, rs2, imm},
                            _ => Instruction::Undefined{
                                instruction,
                                msg: format!("format: B, opcode: {:07b}, func3: {:03b}", opcode, func3)
                            }
                        }
                    },

                    _ => Instruction::Undefined{
                        instruction,
                        msg: format!("format: B, opcode: {:07b}", opcode)
                    }
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
