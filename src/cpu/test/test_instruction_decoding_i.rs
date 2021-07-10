// Unittests to check the decoding of I formatted instructions.

#[cfg(test)]
mod test_instruction_decoding_i {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::{XRegister, FRegister};

    // I format
    // 0b_000000000000_00000_000_00000_0000000

    #[test]
    fn imm_sign_extension_1() {
        let raw_instruction: u32 = 0b_1_11010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::jalr {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b1111111111111111111111111111111111111111111111111111_1_11010110111,
        });
    }

    #[test]
    fn imm_sign_extension_0() {
        let raw_instruction: u32 = 0b_0_11010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::jalr {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b0000000000000000000000000000000000000000000000000000_0_11010110111,
        });
    }

    #[test]
    fn jalr() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_1100111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::jalr {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lb() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lb {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lh() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lh {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lw() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lbu() {
        let raw_instruction: u32 = 0b_011010110111_11010_100_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lbu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lhu() {
        let raw_instruction: u32 = 0b_011010110111_11010_101_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lhu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn addi() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::addi {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn slti() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::slti {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn sltiu() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::sltiu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn xori() {
        let raw_instruction: u32 = 0b_011010110111_11010_100_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::xori {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ori() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ori {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn andi() {
        let raw_instruction: u32 = 0b_011010110111_11010_111_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::andi {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn lwu() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::lwu {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ld() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0000011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ld {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn addiw() {
        let raw_instruction: u32 = 0b_011010110111_11010_000_01110_0011011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::addiw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn fence_i() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_0001111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fence_i {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrw() {
        let raw_instruction: u32 = 0b_011010110111_11010_001_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrw {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrs() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrs {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrc() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrc {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrwi() {
        let raw_instruction: u32 = 0b_011010110111_11010_101_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrwi {
            rd: XRegister::x14,
            uimm: 0b11010,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrsi() {
        let raw_instruction: u32 = 0b_011010110111_11010_110_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrsi {
            rd: XRegister::x14,
            uimm: 0b11010,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn csrrci() {
        let raw_instruction: u32 = 0b_001010110111_11010_111_01110_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::csrrci {
            rd: XRegister::x14,
            uimm: 0b11010,
            imm: 0b001010110111,
        });
    }

    #[test]
    fn flw() {
        let raw_instruction: u32 = 0b_011010110111_11010_010_01110_0000111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::flw {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn fld() {
        let raw_instruction: u32 = 0b_011010110111_11010_011_01110_0000111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fld {
            rd: FRegister::f14,
            rs1: FRegister::f26,
            imm: 0b011010110111,
        });
    }

    #[test]
    fn ecall() {
        let raw_instruction: u32 = 0b_000000000000_00000_000_00000_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ecall);
    }

    #[test]
    fn ebreak() {
        let raw_instruction: u32 = 0b_000000000001_00000_000_00000_1110011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::ebreak);
    }

    #[test]
    fn slli() {
        let raw_instruction: u32 = 0b_000000_110111_11010_001_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::slli {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b110111,
        });
    }

    #[test]
    fn srli() {
        let raw_instruction: u32 = 0b_000000_010111_11010_101_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::srli {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b010111,
        });
    }

    #[test]
    fn srai() {
        let raw_instruction: u32 = 0b_010000_110111_11010_101_01110_0010011;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::srai {
            rd: XRegister::x14,
            rs1: XRegister::x26,
            shamt: 0b110111,
        });
    }

    #[test]
    fn fence_tso() {
        //                         0b_0100_0011_0111_00000_00001110_0010011
        let raw_instruction: u32 = 0b_1000_0011_0011_00000_000_00000_0001111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::fence_tso);
    }

    #[test]
    fn pause() {
        //                         0b_0100_0011_0111_00000_00001110_0010011
        let raw_instruction: u32 = 0b_0000_0001_0000_00000_000_00000_0001111;
        let instruction = Instruction::decode(raw_instruction);
        assert_eq!(instruction, Instruction::pause);
    }

}
