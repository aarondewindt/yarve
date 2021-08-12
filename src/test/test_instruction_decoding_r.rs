
#[cfg(test)]
mod test_instruction_decoding_r {
    use crate::cpu::instruction::{Instruction, RoundingMode};
    use crate::cpu::register::{XRegister, FRegister};


    #[test]
    fn add() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::add {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sub() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sub {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn xor() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_100_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::xor {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn or() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_110_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::or {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn and() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_111_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::and {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sll() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sll {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn srl() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::srl {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sra() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sra {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn slt() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_010_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::slt {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sltu() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_011_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sltu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn addw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::addw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn subw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::subw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sllw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sllw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn srlw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::srlw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn sraw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sraw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn slliw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_001_01110_0011011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::slliw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn srliw() {
        let raw_instruction: u32 = 0b_0000000_10111_11010_101_01110_0011011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::srliw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn sraiw() {
        let raw_instruction: u32 = 0b_0100000_10111_11010_101_01110_0011011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sraiw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b10111,
        });
    }

    #[test]
    fn mul() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_000_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::mul {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn mulh() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_001_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::mulh {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn mulhsu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_010_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::mulhsu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn mulhu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_011_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::mulhu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn div() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_100_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::div {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn divu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_101_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::divu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn rem() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_110_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::rem {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn remu() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_111_01110_0110011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::remu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn mulw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_000_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::mulw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn divw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_100_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::divw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn divuw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_101_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::divuw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn remw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_110_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::remw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn remuw() {
        let raw_instruction: u32 = 0b_0000001_10111_11010_111_01110_0111011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::remuw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
        });
    }

    #[test]
    fn lr_w() {
        let raw_instruction: u32 = 0b_00010_0_1_00000_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lr_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn lr_d() {
        let raw_instruction: u32 = 0b_00010_0_1_00000_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lr_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn sc_w() {
        let raw_instruction: u32 = 0b_00011_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sc_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoswap_w() {
        let raw_instruction: u32 = 0b_00001_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoswap_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoadd_w() {
        let raw_instruction: u32 = 0b_00000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoadd_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoxor_w() {
        let raw_instruction: u32 = 0b_00100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoxor_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoand_w() {
        let raw_instruction: u32 = 0b_01100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoand_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoor_w() {
        let raw_instruction: u32 = 0b_01000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoor_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomin_w() {
        let raw_instruction: u32 = 0b_10000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomin_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomax_w() {
        let raw_instruction: u32 = 0b_10100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomax_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amominu_w() {
        let raw_instruction: u32 = 0b_11000_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amominu_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomaxu_w() {
        let raw_instruction: u32 = 0b_11100_0_1_10111_11010_010_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomaxu_w {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn sc_d() {
        let raw_instruction: u32 = 0b_00011_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::sc_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    // (0b011, 0b00001, rs2) => Instruction::amoswap_d{rd, rs1, rs2, rl, aq},
    #[test]
    fn amoswap_d() {
        let raw_instruction: u32 = 0b_00001_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoswap_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoadd_d() {
        let raw_instruction: u32 = 0b_00000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoadd_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoxor_d() {
        let raw_instruction: u32 = 0b_00100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoxor_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoand_d() {
        let raw_instruction: u32 = 0b_01100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoand_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amoor_d() {
        let raw_instruction: u32 = 0b_01000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amoor_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomin_d() {
        let raw_instruction: u32 = 0b_10000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomin_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomax_d() {
        let raw_instruction: u32 = 0b_10100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomax_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amominu_d() {
        let raw_instruction: u32 = 0b_11000_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amominu_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn amomaxu_d() {
        let raw_instruction: u32 = 0b_11100_0_1_10111_11010_011_01110_0101111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::amomaxu_d {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            rl: true,
            aq: false,
        });
    }

    #[test]
    fn fmadd_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1000011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmadd_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f28
        });
    }

    // fmsub_s
    #[test]
    fn fmsub_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1000111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmsub_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f28
        });
    }

    // fnmsub_s
    #[test]
    fn fnmsub_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1001011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fnmsub_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f28
        });
    }

    // fnmadd_s
    #[test]
    fn fnmadd_s() {
        let raw_instruction: u32 = 0b_11100_00_10111_11010_011_01110_1001111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fnmadd_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f28
        });
    }

    // fadd_s
    #[test]
    fn fadd_s() {
        let raw_instruction: u32 = 0b_00000_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fadd_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsub_s
    #[test]
    fn fsub_s() {
        let raw_instruction: u32 = 0b_00001_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsub_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmul_s
    #[test]
    fn fmul_s() {
        let raw_instruction: u32 = 0b_00010_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmul_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fdiv_s
    #[test]
    fn fdiv_s() {
        let raw_instruction: u32 = 0b_00011_00_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fdiv_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsqrt_s
    #[test]
    fn fsqrt_s() {
        let raw_instruction: u32 = 0b_01011_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsqrt_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fsgnj_s
    #[test]
    fn fsgnj_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnj_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsgnjn_s
    #[test]
    fn fsgnjn_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnjn_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsgnjx_s
    #[test]
    fn fsgnjx_s() {
        let raw_instruction: u32 = 0b_00100_00_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnjx_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmin_s
    #[test]
    fn fmin_s() {
        let raw_instruction: u32 = 0b_00101_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmin_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmax_s
    #[test]
    fn fmax_s() {
        let raw_instruction: u32 = 0b_00101_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmax_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fcvt_w_s
    #[test]
    fn fcvt_w_s() {
        let raw_instruction: u32 = 0b_11000_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_w_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_wu_s
    #[test]
    fn fcvt_wu_s() {
        let raw_instruction: u32 = 0b_11000_00_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_wu_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fmv_x_w
    #[test]
    fn fmv_x_w() {
        let raw_instruction: u32 = 0b_11100_00_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmv_x_w {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }

    // feq_s
    #[test]
    fn feq_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::feq_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // flt_s
    #[test]
    fn flt_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::flt_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fle_s
    #[test]
    fn fle_s() {
        let raw_instruction: u32 = 0b_10100_00_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fle_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fclass_s
    #[test]
    fn fclass_s() {
        let raw_instruction: u32 = 0b_11100_00_00000_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fclass_s {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }

    // fcvt_s_w
    #[test]
    fn fcvt_s_w() {
        let raw_instruction: u32 = 0b_11010_00_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_s_w {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_s_wu
    #[test]
    fn fcvt_s_wu() {
        let raw_instruction: u32 = 0b_11010_00_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_s_wu {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fmv_w_x
    #[test]
    fn fmv_w_x() {
        let raw_instruction: u32 = 0b_11110_00_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmv_w_x {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }

    // fcv_tl_s
    #[test]
    fn fcv_tl_s() {
        let raw_instruction: u32 = 0b_11000_00_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcv_tl_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcv_tlu_s
    #[test]
    fn fcv_tlu_s() {
        let raw_instruction: u32 = 0b_11000_00_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcv_tlu_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcv_ts_l
    #[test]
    fn fcv_ts_l() {
        let raw_instruction: u32 = 0b_11010_00_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcv_ts_l {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcv_ts_lu
    #[test]
    fn fcv_ts_lu() {
        let raw_instruction: u32 = 0b_11010_00_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcv_ts_lu {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    #[test]
    fn fsw() {
        let raw_instruction: u32 = 0b_01010_00_01011_11010_010_01110_0100111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsw {
            imm: 0b01010_00_01110,
            rs1: FRegister::f26,
            rs2: FRegister::f11,
        });
    }

    #[test]
    #[allow(overflowing_literals)]
    fn fsw_sign_extend() {
        let raw_instruction: u32 = 0b_11010_00_01011_11010_010_01110_0100111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsw {
            imm: 0xFFFFFFFFFFFFFD0E,
            rs1: FRegister::f26,
            rs2: FRegister::f11,
        });
    }

    #[test]
    fn fmadd_d() {
        let raw_instruction: u32 = 0b_01100_01_10111_11010_011_01110_1000011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmadd_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f12
        });
    }

    // fmsub_d
    #[test]
    fn fmsub_d() {
        let raw_instruction: u32 = 0b_01100_01_10111_11010_011_01110_1000111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmsub_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f12
        });
    }

    // fnmsub_d
    #[test]
    fn fnmsub_d() {
        let raw_instruction: u32 = 0b_01100_01_10111_11010_011_01110_1001011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fnmsub_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f12
        });
    }

    // fnmadd_d
    #[test]
    fn fnmadd_d() {
        let raw_instruction: u32 = 0b_01100_01_10111_11010_011_01110_1001111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fnmadd_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
            rs3: FRegister::f12
        });
    }

    // fadd_d
    #[test]
    fn fadd_d() {
        let raw_instruction: u32 = 0b_00000_01_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fadd_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsub_d
    #[test]
    fn fsub_d() {
        let raw_instruction: u32 = 0b_00001_01_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsub_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmul_d
    #[test]
    fn fmul_d() {
        let raw_instruction: u32 = 0b_00010_01_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmul_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fdiv_d
    #[test]
    fn fdiv_d() {
        let raw_instruction: u32 = 0b_00011_01_10111_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fdiv_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsqrt_d
    #[test]
    fn fsqrt_d() {
        let raw_instruction: u32 = 0b_01011_01_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsqrt_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fsgnj_d
    #[test]
    fn fsgnj_d() {
        let raw_instruction: u32 = 0b_00100_01_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnj_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsgnjn_d
    #[test]
    fn fsgnjn_d() {
        let raw_instruction: u32 = 0b_00100_01_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnjn_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fsgnjx_d
    #[test]
    fn fsgnjx_d() {
        let raw_instruction: u32 = 0b_00100_01_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fsgnjx_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmin_d
    #[test]
    fn fmin_d() {
        let raw_instruction: u32 = 0b_00101_01_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmin_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fmax_d
    #[test]
    fn fmax_d() {
        let raw_instruction: u32 = 0b_00101_01_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmax_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fcvt_s_d
    #[test]
    fn fcvt_s_d() {
        let raw_instruction: u32 = 0b_01000_01_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_s_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_d_s
    #[test]
    fn fcvt_d_s() {
        let raw_instruction: u32 = 0b_01000_01_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_d_s {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // feq_d
    #[test]
    fn feq_d() {
        let raw_instruction: u32 = 0b_10100_01_10111_11010_010_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::feq_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // flt_d
    #[test]
    fn flt_d() {
        let raw_instruction: u32 = 0b_10100_01_10111_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::flt_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fle_d
    #[test]
    fn fle_d() {
        let raw_instruction: u32 = 0b_10100_01_10111_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fle_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            rs2: FRegister::f23,
        });
    }

    // fclass_d
    #[test]
    fn fclass_d() {
        let raw_instruction: u32 = 0b_11100_01_00000_11010_001_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fclass_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }

    // fcvt_w_d
    #[test]
    fn fcvt_w_d() {
        let raw_instruction: u32 = 0b_11000_01_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_w_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_wu_d
    #[test]
    fn fcvt_wu_d() {
        let raw_instruction: u32 = 0b_11000_01_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_wu_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_d_w
    #[test]
    fn fcvt_d_w() {
        let raw_instruction: u32 = 0b_11010_01_00000_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_d_w {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_d_wu
    #[test]
    fn fcvt_d_wu() {
        let raw_instruction: u32 = 0b_11010_01_00001_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_d_wu {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_l_d
    #[test]
    fn fcvt_l_d() {
        let raw_instruction: u32 = 0b_11000_01_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_l_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_lu_d
    #[test]
    fn fcvt_lu_d() {
        let raw_instruction: u32 = 0b_11000_01_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_lu_d {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fmv_x_d
    #[test]
    fn fmv_x_d() {
        let raw_instruction: u32 = 0b_11100_01_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmv_x_d {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }

    // fcvt_d_l
    #[test]
    fn fcvt_d_l() {
        let raw_instruction: u32 = 0b_11010_01_00010_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_d_l {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fcvt_d_lu
    #[test]
    fn fcvt_d_lu() {
        let raw_instruction: u32 = 0b_11010_01_00011_11010_011_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fcvt_d_lu {
            rd: FRegister::f14,
            rm: RoundingMode::rup,
            rs1: FRegister::f26,
        });
    }

    // fmv_d_x
    #[test]
    fn fmv_d_x() {
        let raw_instruction: u32 = 0b_11110_01_00000_11010_000_01110_1010011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::fmv_d_x {
            rd: FRegister::f14,
            rs1: FRegister::f26,
        });
    }


}
