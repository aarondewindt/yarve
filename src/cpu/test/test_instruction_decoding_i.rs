// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_i {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::Register;

    // I format
    // 0b_000000000000_00000_000_00000_0000000

    #[test]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1_11010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::JALR {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b1111111111111111111111111111111111111111111111111111_1_11010110111,
        });
    }

    #[test]
    fn imm_sign_extension_0() {
        let raw_instruction: u32 = 0b_0_11010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::JALR {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b0000000000000000000000000000000000000000000000000000_0_11010110111,
        });
    }

    #[test]
    fn jalr() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::JALR {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lb() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LB {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lh() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LH {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lw() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LW {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lbu() {
        let raw_instruction: u32 = 0b_011010110111_11010_100_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LBU {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lhu() {
        let raw_instruction: u32 = 0b_011010110111_11010_101_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LHU {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn addi() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ADDI {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn slti() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::SLTI {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn sltiu() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::SLTIU {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn xori() {
        let raw_instruction: u32 = 0b_011010110111_11010_100_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::XORI {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ori() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ORI {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn andi() {
        let raw_instruction: u32 = 0b_011010110111_11010_111_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ANDI {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lwu() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LWU {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ld() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::LD {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn addiw() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0011011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ADDIW {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn fence_i() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_0001111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::FENCE_I {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrw() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRW {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrs() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRS {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrc() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRC {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrwi() {
        let raw_instruction: u32 = 0b_011010110111_11010_101_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRWI {
            rd: Register::from(0b01110usize),
            uimm: 0b11010,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrsi() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRSI {
            rd: Register::from(0b01110usize),
            uimm: 0b11010,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrci() {
        let raw_instruction: u32 = 0b_011010110111_11010_111_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::CSRRCI {
            rd: Register::from(0b01110usize),
            uimm: 0b11010,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn flw() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0000111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::FLW {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn fld() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0000111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::FLD {
            rd: Register::from(0b01110usize),
            rs1: Register::from(0b11010usize),
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ecall() {
        let raw_instruction: u32 = 0b_000000000000_00000_000_00000_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ECALL);
    }

    #[test]
    fn ebreak() {
        let raw_instruction: u32 = 0b_000000000001_00000_000_00000_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::EBREAK);
    }
}
