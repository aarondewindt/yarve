
#[cfg(test)]
mod test_core {
    use crate::cpu::register::XRegister;
    use crate::cpu::core::Core;
    use crate::bus::Bus;
    use crate::dram::DRAM;
    use crate::device::Device;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::endianness::Endianness;

    fn new_test_core() -> Core {
        let dram = DRAM::new(16);
        Core::new(Rc::new(RefCell::new(Bus::new(
            vec![(0 , Box::new(dram))]
        ))))
    }

    #[test]
    fn test_zero_register() {
        let mut core = new_test_core();

        assert_eq!(core.x_registers[XRegister::x0], 0);

        core.x_registers[XRegister::x0] = 0;
        assert_eq!(core.x_registers[XRegister::x0], 0);

        core.x_registers[XRegister::x0] = 1;
        assert_eq!(core.x_registers[XRegister::x0], 0);
    }

    #[test]
    fn test_execute() {
        let mut core = new_test_core();
        {
            let mut bus = core.bus.borrow_mut();
            bus.write_int(
                0x00,
                0b_0000000_10111_11010_000_01110_0110011,
                4,
                Endianness::LittleEndian
            ).unwrap();
            bus.write_int(
                0x04,
                0b_0100000_10111_11010_000_01110_0110011,
                4,
                Endianness::LittleEndian
            ).unwrap();
        }
        core.x_registers[XRegister::x26] = 52;
        core.x_registers[XRegister::x23] = 4;
        assert_eq!(core.x_registers[XRegister::x14], 0);

        core.execute().unwrap();
        assert_eq!(core.x_registers[XRegister::x14], 56);

        core.execute().unwrap();
        assert_eq!(core.x_registers[XRegister::x14], 48);
    }
}