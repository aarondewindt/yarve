// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_r {
    use crate::cpu::instruction::{Instruction, RoundingMode};
    use crate::cpu::register::{Register, FloatRegister};

    // I format
    // 0b_0000000_00000_00000_000_00000_0000000

    #[test]
    fn add() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::add {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sub() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sub {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn xor() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_100_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::xor {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn or() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_110_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::or {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn and() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_111_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::and {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sll() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sll {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn srl() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::srl {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sra() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sra {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn slt() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_010_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::slt {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sltu() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_011_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sltu {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn addw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::addw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn subw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::subw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sllw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sllw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn srlw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::srlw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn sraw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sraw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn slliw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0011011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::slliw {
            rd: Register::x14,
            rs1: Register::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn srliw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0011011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::srliw {
            rd: Register::x14,
            rs1: Register::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn sraiw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0011011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sraiw {
            rd: Register::x14,
            rs1: Register::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn mul() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::mul {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn mulh() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_001_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::mulh {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn mulhsu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_010_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::mulhsu {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn mulhu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_011_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::mulhu {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn div() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_100_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::div {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn divu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::divu {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn rem() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_110_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::rem {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn remu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_111_01110_0110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::remu {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn mulw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::mulw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn divw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_100_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::divw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn divuw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::divuw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn remw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_110_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::remw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn remuw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_111_01110_0111011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::remuw {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
        });
    }

    #[test]
    fn lr_w() {
        let raw_instruction: u32 = 0b_00010_0_1_00000_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lr_w {
            rd: Register::x14,
            rs1: Register::x26,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn lr_d() {
        let raw_instruction: u32 = 0b_00010_0_1_00000_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lr_d {
            rd: Register::x14,
            rs1: Register::x26,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn sc_w() {
        let raw_instruction: u32 = 0b_00011_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sc_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoswap_w() {
        let raw_instruction: u32 = 0b_00001_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoswap_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoadd_w() {
        let raw_instruction: u32 = 0b_00000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoadd_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoxor_w() {
        let raw_instruction: u32 = 0b_00100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoxor_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoand_w() {
        let raw_instruction: u32 = 0b_01100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoand_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoor_w() {
        let raw_instruction: u32 = 0b_01000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoor_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomin_w() {
        let raw_instruction: u32 = 0b_10000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomin_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomax_w() {
        let raw_instruction: u32 = 0b_10100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomax_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amominu_w() {
        let raw_instruction: u32 = 0b_11000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amominu_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomaxu_w() {
        let raw_instruction: u32 = 0b_11100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomaxu_w {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn sc_d() {
        let raw_instruction: u32 = 0b_00011_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sc_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    // (0b011, 0b00001, rs2) => Instruction::amoswap_d{rd, rs1, rs2, rl, aq},
    #[test]
    fn amoswap_d() {
        let raw_instruction: u32 = 0b_00001_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoswap_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoadd_d() {
        let raw_instruction: u32 = 0b_00000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoadd_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoxor_d() {
        let raw_instruction: u32 = 0b_00100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoxor_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoand_d() {
        let raw_instruction: u32 = 0b_01100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoand_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoor_d() {
        let raw_instruction: u32 = 0b_01000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amoor_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomin_d() {
        let raw_instruction: u32 = 0b_10000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomin_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomax_d() {
        let raw_instruction: u32 = 0b_10100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomax_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amominu_d() {
        let raw_instruction: u32 = 0b_11000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amominu_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomaxu_d() {
        let raw_instruction: u32 = 0b_11100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::amomaxu_d {
            rd: Register::x14,
            rs1: Register::x26,
            rs2: Register::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn fmadd_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmadd_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
            rs3: FloatRegister::f28
        });
    }

    // fmsub_s
    #[test]
    fn fmsub_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1000111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmsub_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
            rs3: FloatRegister::f28
        });
    }

    // fnmsub_s
    #[test]
    fn fnmsub_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1001011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fnmsub_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
            rs3: FloatRegister::f28
        });
    }

    // fnmadd_s
    #[test]
    fn fnmadd_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1001111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fnmadd_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
            rs3: FloatRegister::f28
        });
    }

    // fadd_s
    #[test]
    fn fadd_s() {
        let raw_instruction: u32 = 0b_00000_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fadd_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fsub_s
    #[test]
    fn fsub_s() {
        let raw_instruction: u32 = 0b_00001_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsub_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fmul_s
    #[test]
    fn fmul_s() {
        let raw_instruction: u32 = 0b_00010_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmul_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fdiv_s
    #[test]
    fn fdiv_s() {
        let raw_instruction: u32 = 0b_00011_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fdiv_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fsqrt_s
    #[test]
    fn fsqrt_s() {
        let raw_instruction: u32 = 0b_01011_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsqrt_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fsgnj_s
    #[test]
    fn fsgnj_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsgnj_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fsgnjn_s
    #[test]
    fn fsgnjn_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsgnjn_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fsgnjx_s
    #[test]
    fn fsgnjx_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsgnjx_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fmin_s
    #[test]
    fn fmin_s() {
        let raw_instruction: u32 = 0b_00101_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmin_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fmax_s
    #[test]
    fn fmax_s() {
        let raw_instruction: u32 = 0b_00101_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmax_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fcvt_w_s
    #[test]
    fn fcvt_w_s() {
        let raw_instruction: u32 = 0b_11000_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcvt_w_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fcvt_wu_s
    #[test]
    fn fcvt_wu_s() {
        let raw_instruction: u32 = 0b_11000_00_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcvt_wu_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fmv_x_w
    #[test]
    fn fmv_x_w() {
        let raw_instruction: u32 = 0b_11100_00_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmv_x_w {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
        });
    }

    // feq_s
    #[test]
    fn feq_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::feq_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // flt_s
    #[test]
    fn flt_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::flt_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fle_s
    #[test]
    fn fle_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fle_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f23,
        });
    }

    // fclass_s
    #[test]
    fn fclass_s() {
        let raw_instruction: u32 = 0b_11100_00_00000_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fclass_s {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
        });
    }

    // fcvt_s_w
    #[test]
    fn fcvt_s_w() {
        let raw_instruction: u32 = 0b_11010_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcvt_s_w {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fcvt_s_wu
    #[test]
    fn fcvt_s_wu() {
        let raw_instruction: u32 = 0b_11010_00_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcvt_s_wu {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fmv_w_x
    #[test]
    fn fmv_w_x() {
        let raw_instruction: u32 = 0b_11110_00_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fmv_w_x {
            rd: FloatRegister::f14,
            rs1: FloatRegister::f26,
        });
    }

    // fcv_tl_s
    #[test]
    fn fcv_tl_s() {
        let raw_instruction: u32 = 0b_11000_00_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcv_tl_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fcv_tlu_s
    #[test]
    fn fcv_tlu_s() {
        let raw_instruction: u32 = 0b_11000_00_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcv_tlu_s {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fcv_ts_l
    #[test]
    fn fcv_ts_l() {
        let raw_instruction: u32 = 0b_11010_00_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcv_ts_l {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    // fcv_ts_lu
    #[test]
    fn fcv_ts_lu() {
        let raw_instruction: u32 = 0b_11010_00_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fcv_ts_lu {
            rd: FloatRegister::f14,
            rm: RoundingMode::rup,
            rs1: FloatRegister::f26,
        });
    }

    #[test]
    fn fsw() {
        let raw_instruction: u32 = 0b_01010_00_01011_11010_010_01110_0100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fsw {
            imm: 0b01010_00_01110,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f11,
        });
    }

    #[test]
    fn fsw_sign_extend() {
        let raw_instruction: u32 = 0b_11010_00_01011_11010_010_01110_0100111;
        let instruction = Instruction::decode(raw_instruction);
        if let Instruction::fsw{imm, rs1, rs2} = instruction {
            println!("{:012b}", imm)
        }

        assert_eq!(instruction, Instruction::fsw {
            imm: 0xFFFFFFFFFFFFFD0E,
            rs1: FloatRegister::f26,
            rs2: FloatRegister::f11,
        });
    }
}
