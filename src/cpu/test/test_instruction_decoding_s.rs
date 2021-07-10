// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_s {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::XRegister;

    // I format
    // 0b_0000000_00000_00000_000_00000_0000000

    #[test]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1110101_10111_11010_000_01110_0100011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sb {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b1111111111111111111111111111111111111111111111111111_1110101_01110
        });
    }

    #[test]
    fn sb() {
        let raw_instruction: u32 = 0b_0110101_10111_11010_000_01110_0100011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sb {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0110101_01110
        });
    }

    #[test]
    fn sh() {
        let raw_instruction: u32 = 0b_0010101_10111_11010_001_01110_0100011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sh {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0010101_01110
        });
    }

    #[test]
    fn sw() {
        let raw_instruction: u32 = 0b_0110101_10111_11010_010_01110_0100011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sw {
            rs1: XRegister::x26,
            rs2: XRegister::x23,
            imm: 0b0110101_01110
        });
    }


}
