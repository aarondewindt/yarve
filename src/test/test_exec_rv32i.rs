// I'll test most instructions using the RISC-V official tests.
// These are just to get me to the point where I can actually run those
// tests.

#[cfg(test)]
mod test_rv32i {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::register::XRegister;
    use crate::cpu::core::Core;

    use crate::bus::Bus;
    use crate::dram::DRAM;
    use crate::device::Device;
    use std::rc::Rc;
    use std::cell::RefCell;


    fn new_test_core() -> Core {
        let dram = DRAM::new(16);
        Core::new(Rc::new(RefCell::new(Bus::new(
            vec![(0 , Box::new(dram))]
        ))))
    }

    #[test]
    fn test_add() {
        let mut core = new_test_core();

        let instruction = Instruction::add{
            rd: XRegister::x3,
            rs1: XRegister::x1,
            rs2: XRegister::x2
        };

        core.x_registers[XRegister::x1] = 2;
        core.x_registers[XRegister::x2] = 4;
        instruction.execute(&mut core).unwrap();
        assert_eq!(core.x_registers[XRegister::x3], 6);

        core.x_registers[XRegister::x1] = u64::MAX - 2;
        core.x_registers[XRegister::x2] = 6;
        instruction.execute(&mut core).unwrap();
        assert_eq!(core.x_registers[XRegister::x3], 3);
    }

    #[test]
    fn test_lb() {
        let mut core = new_test_core();

        core.bus.borrow_mut().write_bytes(0, &[
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]).unwrap();

        Instruction::lb {
            rd: XRegister::x1,
            rs1: XRegister::x0,
            imm: 4,
        }.execute(&mut core).unwrap();
        assert_eq_hex!(core.x_registers[XRegister::x1], 0x14);

        core.x_registers[XRegister::x2] = 3;
        Instruction::lb {
            rd: XRegister::x1,
            rs1: XRegister::x2,
            imm: 4,
        }.execute(&mut core).unwrap();
        assert_eq_hex!(core.x_registers[XRegister::x1], 0x17);

        core.x_registers[XRegister::x2] = 3;
        Instruction::lb {
            rd: XRegister::x1,
            rs1: XRegister::x2,
            imm: -2,
        }.execute(&mut core).unwrap();
        assert_eq_hex!(core.x_registers[XRegister::x1], 0x11);
    }


}