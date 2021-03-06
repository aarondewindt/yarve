
#[cfg(test)]
mod test_instruction_decoding_b {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::XRegister;

    #[test]
    #[allow(overflowing_literals)]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1_110101_10111_11010_000_0111_0_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::beq {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b111111111111111111111111111111111111111111111111111_1_0_110101_0111_0
        });
    }

    #[test]
    fn beq() {
        let raw_instruction: u32 = 0b_0_110101_10111_11010_000_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::beq {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_110101_0111_0
        });
    }

    #[test]
    fn bne() {
        let raw_instruction: u32 = 0b_0_010101_10111_11010_001_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::bne {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_010101_0111_0
        });
    }

    #[test]
    fn blt() {
        let raw_instruction: u32 = 0b_0_110101_10111_11010_100_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::blt {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_110101_0111_0
        });
    }

    #[test]
    fn bge() {
        let raw_instruction: u32 = 0b_0_110101_10111_11010_101_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::bge {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_110101_0111_0
        });
    }

    #[test]
    fn bltu() {
        let raw_instruction: u32 = 0b_0_110101_10111_11010_110_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::bltu {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_110101_0111_0
        });
    }

    #[test]
    fn bgeu() {
        let raw_instruction: u32 = 0b_0_110101_10111_11010_111_0111_1_1100011;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::bgeu {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0_1_110101_0111_0
        });
    }


}
