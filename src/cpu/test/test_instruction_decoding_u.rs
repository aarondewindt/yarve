// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_i {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::Register;

    // U format
    // 0b_00000000000000000000_00000_0000000

    #[test]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1_1101011011111010011_01110_0110111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LUI {
            rd: Register::from(0b01110usize),
            imm: 0b11111111111111111111111111111111_1_1101011011111010011_000000000000,
        });
    }

    #[test]
    fn imm_sign_extension_0() {
        let raw_instruction: u32 = 0b_0_1101011011111010011_01110_0110111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LUI {
            rd: Register::from(0b01110usize),
            imm:  0b00000000000000000000000000000000_0_1101011011111010011_000000000000,
        });
    }

    #[test]
    fn lui() {
        let raw_instruction: u32 = 0b_01101011011111010000_01110_0110111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LUI {
            rd: Register::from(0b01110usize),
            imm: 0b01101011011111010000_000000000000,
        });
    }

    #[test]
    fn auipc() {
        let raw_instruction: u32 = 0b_01101011011111010000_01110_0010111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::AUIPC {
            rd: Register::from(0b01110usize),
            imm: 0b01101011011111010000_000000000000,
        });
    }
}