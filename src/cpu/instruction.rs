use crate::cpu::register::{XRegister, FRegister};


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
         // In Rounding Mode XRegister, reserved.
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

pub enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    // RV32I Base Instruction Set
    // R: 0110011
    add {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sub {rd: XRegister, rs1: XRegister, rs2: XRegister},
    xor {rd: XRegister, rs1: XRegister, rs2: XRegister},
    or {rd: XRegister, rs1: XRegister, rs2: XRegister},
    and {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sll {rd: XRegister, rs1: XRegister, rs2: XRegister},
    srl {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sra {rd: XRegister, rs1: XRegister, rs2: XRegister},
    slt {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sltu {rd: XRegister, rs1: XRegister, rs2: XRegister},

    // I: 0010011
    addi {rd: XRegister, rs1: XRegister, imm: i64},
    xori {rd: XRegister, rs1: XRegister, imm: u64},
    ori {rd: XRegister, rs1: XRegister, imm: u64},
    andi {rd: XRegister, rs1: XRegister, imm: u64},
    slli {rd: XRegister, rs1: XRegister, shamt: i64},
    srli {rd: XRegister, rs1: XRegister, shamt: i64},
    srai {rd: XRegister, rs1: XRegister, shamt: i64},
    slti {rd: XRegister, rs1: XRegister, imm: i64},
    sltiu {rd: XRegister, rs1: XRegister, imm: u64},

    // I: 0000011
    lb {rd: XRegister, rs1: XRegister, imm: isize},
    lh {rd: XRegister, rs1: XRegister, imm: isize},
    lw {rd: XRegister, rs1: XRegister, imm: isize},
    lbu {rd: XRegister, rs1: XRegister, imm: isize},
    lhu {rd: XRegister, rs1: XRegister, imm: isize},

    // S: 0100011
    sb {rs1: XRegister, rs2: XRegister, imm: isize},
    sh {rs1: XRegister, rs2: XRegister, imm: isize},
    sw {rs1: XRegister, rs2: XRegister, imm: isize},

    // B: 1100011
    beq {rs1: XRegister, rs2: XRegister, imm: isize},
    bne {rs1: XRegister, rs2: XRegister, imm: isize},
    blt {rs1: XRegister, rs2: XRegister, imm: isize},
    bge {rs1: XRegister, rs2: XRegister, imm: isize},
    bltu {rs1: XRegister, rs2: XRegister, imm: isize},
    bgeu {rs1: XRegister, rs2: XRegister, imm: isize},

    jal {rd: XRegister, imm: isize},  // J: 1101111
    jalr {rd: XRegister, rs1: XRegister, imm: isize},  // I: 1100111

    lui {rd: XRegister, uimm: u64},  // U: 0110111
    auipc {rd: XRegister, uimm: u64},  // U: 0010111

    // I: 1110011
    ecall,
    ebreak,

    // ?: 0001111
    fence {rd: XRegister, rs1: XRegister, succ: u64, pred: u64, fm: u64},
    fence_tso,
    pause,

    // RV64I Base Instruction Set
    // I: 0000011
    lwu {rd: XRegister, rs1: XRegister, imm: isize},
    ld {rd: XRegister, rs1: XRegister, imm: isize},

    // R: 0100011
    sd {rs1: XRegister, rs2: XRegister, imm: isize},

    // I: 0011011
    addiw {rd: XRegister, rs1: XRegister, imm: i64},

    // R: 0011011
    slliw {rd: XRegister, rs1: XRegister, shamt: i64},
    srliw {rd: XRegister, rs1: XRegister, shamt: i64},
    sraiw {rd: XRegister, rs1: XRegister, shamt: i64},

    // R: 0111011
    addw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    subw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sllw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    srlw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    sraw {rd: XRegister, rs1: XRegister, rs2: XRegister},

    // RV32/RV64 Zifencei Standard Extension
    fence_i {rd: XRegister, rs1: XRegister, imm: i64},

    // RV32/RV64 Zicsr Standard Extension
    csrrw {rd: XRegister, rs1: XRegister, imm: i64},
    csrrs {rd: XRegister, rs1: XRegister, imm: i64},
    csrrc {rd: XRegister, rs1: XRegister, imm: i64},
    csrrwi {rd: XRegister, uimm: u64, imm: i64},
    csrrsi {rd: XRegister, uimm: u64, imm: i64},
    csrrci {rd: XRegister, uimm: u64, imm: i64},

    // RV32M Standard Extension
    // R: 0110011
    mul {rd: XRegister, rs1: XRegister, rs2: XRegister},
    mulh {rd: XRegister, rs1: XRegister, rs2: XRegister},
    mulhsu {rd: XRegister, rs1: XRegister, rs2: XRegister},
    mulhu {rd: XRegister, rs1: XRegister, rs2: XRegister},
    div {rd: XRegister, rs1: XRegister, rs2: XRegister},
    divu {rd: XRegister, rs1: XRegister, rs2: XRegister},
    rem {rd: XRegister, rs1: XRegister, rs2: XRegister},
    remu {rd: XRegister, rs1: XRegister, rs2: XRegister},

    // RV64M Standard Extension
    mulw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    divw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    divuw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    remw {rd: XRegister, rs1: XRegister, rs2: XRegister},
    remuw {rd: XRegister, rs1: XRegister, rs2: XRegister},

    // RV32A Standard Extension
    // R: 0101111
    lr_w {rd: XRegister, rs1: XRegister, rl: bool, aq: bool},
    sc_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoswap_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoadd_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoxor_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoand_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoor_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomin_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomax_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amominu_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomaxu_w {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},

    // RV64A Standard Extension
    lr_d {rd: XRegister, rs1: XRegister, rl: bool, aq: bool},
    sc_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoswap_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoadd_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoxor_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoand_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amoor_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomin_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomax_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amominu_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},
    amomaxu_d {rd: XRegister, rs1: XRegister, rs2: XRegister, rl: bool, aq: bool},

    // RV32F Standard Extension
    // I: 0000111
    flw {rd: FRegister, rs1: FRegister, imm: i64},

    // R: 0100111
    fsw {imm: i64, rs1: FRegister, rs2: FRegister},

    // R4: 1000011
    fmadd_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},

    // R4: 1000111
    fmsub_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},

    // R4: 1001011
    fnmsub_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},

    // R4: 1001111
    fnmadd_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},

    // R: 1010011
    fadd_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fsub_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fmul_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fdiv_s {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fsqrt_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fsgnj_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fsgnjn_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fsgnjx_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fmin_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fmax_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fcvt_w_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_wu_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fmv_x_w {rd: FRegister, rs1: FRegister},
    feq_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    flt_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fle_s {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fclass_s {rd: FRegister, rs1: FRegister},
    fcvt_s_w {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_s_wu {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fmv_w_x {rd: FRegister, rs1: FRegister},

    // RV64F Standard Extension
    // R: 1010011
    fcv_tl_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcv_tlu_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcv_ts_l {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcv_ts_lu {rd: FRegister, rm: RoundingMode, rs1: FRegister},

    // RV32D Standard Extension
    fld {rd: FRegister, rs1: FRegister, imm: i64},
    fsd {imm: i64, rs1: FRegister, rs2: FRegister},
    fmadd_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},
    fmsub_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},
    fnmsub_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},
    fnmadd_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister, rs3: FRegister},
    fadd_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fsub_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fmul_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fdiv_d {rd: FRegister, rm: RoundingMode, rs1: FRegister, rs2: FRegister},
    fsqrt_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fsgnj_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fsgnjn_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fsgnjx_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fmin_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fmax_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fcvt_s_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_d_s {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    feq_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    flt_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fle_d {rd: FRegister, rs1: FRegister, rs2: FRegister},
    fclass_d {rd: FRegister, rs1: FRegister},
    fcvt_w_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_wu_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_d_w {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_d_wu {rd: FRegister, rm: RoundingMode, rs1: FRegister},

    // RV64D Standard Extension
    fcvt_l_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_lu_d {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fmv_x_d {rd: FRegister, rs1: FRegister},
    fcvt_d_l {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fcvt_d_lu {rd: FRegister, rm: RoundingMode, rs1: FRegister},
    fmv_d_x {rd: FRegister, rs1: FRegister},

}
