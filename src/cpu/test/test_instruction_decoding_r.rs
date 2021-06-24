// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_r {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::Register;

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



}
