#[cfg(test)]
mod test_instruction_decoding_u {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::XRegister;

    #[test]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1_1101011011111010011_01110_0110111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lui {
            rd: XRegister::x14,
            uimm: 0b11111111111111111111111111111111_1_1101011011111010011_000000000000,
        });
    }

    #[test]
    fn imm_sign_extension_0() {
        let raw_instruction: u32 = 0b_0_1101011011111010011_01110_0110111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lui {
            rd: XRegister::x14,
            uimm:  0b00000000000000000000000000000000_0_1101011011111010011_000000000000,
        });
    }

    #[test]
    fn lui() {
        let raw_instruction: u32 = 0b_01101011011111010000_01110_0110111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lui {
            rd: XRegister::x14,
            uimm: 0b01101011011111010000_000000000000,
        });
    }

    #[test]
    fn lui2() {
        let raw_instruction: u32 = 0b_00101011011111010000_01110_0110111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::lui {
            rd: XRegister::x14,
            uimm: 0b00101011011111010000_000000000000,
        });
    }

    #[test]
    fn auipc() {
        let raw_instruction: u32 = 0b_01101011011111010000_01110_0010111;
        let instruction = Instruction::decode(raw_instruction).unwrap();
        assert_eq!(instruction, Instruction::auipc {
            rd: XRegister::x14,
            imm: 0b01101011011111010000_000000000000,
        });
    }
}