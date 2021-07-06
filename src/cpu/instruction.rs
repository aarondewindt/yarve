use crate::cpu::register::{Register, FloatRegister};


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
    fsd {imm: u64, rs1: FloatRegister, rs2: FloatRegister},
    fmadd_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},
    fmsub_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},
    fnmsub_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},
    fnmadd_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister, rs3: FloatRegister},
    fadd_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fsub_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fmul_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fdiv_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister, rs2: FloatRegister},
    fsqrt_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fsgnj_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fsgnjn_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fsgnjx_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fmin_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fmax_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fcvt_s_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_d_s {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    feq_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    flt_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fle_d {rd: FloatRegister, rs1: FloatRegister, rs2: FloatRegister},
    fclass_d {rd: FloatRegister, rs1: FloatRegister},
    fcvt_w_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_wu_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_d_w {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_d_wu {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},

    // RV64D Standard Extension
    fcvt_l_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_lu_d {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fmv_x_d {rd: FloatRegister, rs1: FloatRegister},
    fcvt_d_l {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fcvt_d_lu {rd: FloatRegister, rm: RoundingMode, rs1: FloatRegister},
    fmv_d_x {rd: FloatRegister, rs1: FloatRegister},

}
