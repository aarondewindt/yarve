// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_i {
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
}
