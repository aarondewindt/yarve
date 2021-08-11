
#[cfg(test)]
mod test_bus {
    use crate::bus::Bus;
    use crate::dram::DRAM;
    use crate::device::Device;
    use crate::endianness::Endianness;

    #[test]
    fn test_bus_get_device_and_get_address_space_size() {
        let mut dram = DRAM::new(16);

        match dram.write_bytes(0, &[
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        let bus = Bus::new(
            vec![(5 , Box::new(dram))]
        );

        assert_eq!(bus.get_address_space_size(), 21);

        let (address_range, device) = bus.get_device(5).unwrap();
        assert_eq!(*address_range, 5..21);
        assert_eq!(device.read_int(0, 8, Endianness::LittleEndian).unwrap(),
                   0x17_16_15_14_13_12_11_10
        );

        let (address_range, device) = bus.get_device(20).unwrap();
        assert_eq!(*address_range, 5..21);
        assert_eq!(device.read_int(15, 1, Endianness::LittleEndian).unwrap(),
                   0x1F
        );

        match bus.get_device(21) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }

        match bus.get_device(4) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }
    }

    #[test]
    fn test_read_bytes() {
        let mut dram = DRAM::new(16);

        match dram.write_bytes(0, &[
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        let bus = Bus::new(
            vec![(5 , Box::new(dram))]
        );

        let bytes = bus.read_bytes(8, 5).unwrap();
        assert_eq!(bytes, [0x13, 0x14, 0x15, 0x16, 0x17]);

        let bytes = bus.read_bytes(16, 5).unwrap();
        assert_eq!(bytes, [0x1B, 0x1C, 0x1D, 0x1E, 0x1F]);

        let bytes = bus.read_bytes(5, 5).unwrap();
        assert_eq!(bytes, [0x10, 0x11, 0x12, 0x13, 0x14]);

        match bus.read_bytes(17, 5) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }

        match bus.read_bytes(4, 5) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }
    }

    #[test]
    fn test_write_bytes() {
        let mut dram = DRAM::new(16);

        match dram.write_bytes(0, &[
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        let mut bus = Bus::new(
            vec![(5, Box::new(dram))]
        );

        bus.write_bytes(6, &[0x21, 0x22, 0x23, 0x24, 0x25]).unwrap();
        assert_eq!(
            bus.read_bytes(5, 16).unwrap(),
            [
                0x10, 0x21, 0x22, 0x23, 0x24, 0x25, 0x16, 0x17,
                0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F
            ]
        );

        bus.write_bytes(18, &[0x3D, 0x3E, 0x3F]).unwrap();
        assert_eq!(
            bus.read_bytes(5, 16).unwrap(),
            [
                0x10, 0x21, 0x22, 0x23, 0x24, 0x25, 0x16, 0x17,
                0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x3D, 0x3E, 0x3F
            ]
        );

        match bus.write_bytes(17, &[0x21, 0x22, 0x23, 0x24, 0x25]) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }

        match bus.write_bytes(4, &[0x21, 0x22, 0x23, 0x24, 0x25]) {
            Ok(_) => {panic!("PANIC")},
            Err(_) => {}
        }
    }

    #[test]
    fn test_read_int() {
        let mut dram = DRAM::new(16);

        match dram.write_bytes(0, &[
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        let bus = Bus::new(
            vec![(5, Box::new(dram))]
        );

        assert_eq!(0x11, bus.read_int(6, 1, Endianness::LittleEndian).unwrap());
        assert_eq!(0x14_13_12_11, bus.read_int(6, 4, Endianness::LittleEndian).unwrap());
        assert_eq!(0x11, bus.read_int(6, 1, Endianness::BigEndian).unwrap());
        assert_eq!(0x11_12_13_14, bus.read_int(6, 4, Endianness::BigEndian).unwrap());
    }

    #[test]
    fn test_write_int() {
        let mut dram = DRAM::new(16);

        match dram.write_bytes(0, &[
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F]) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        let mut bus = Bus::new(
            vec![(5, Box::new(dram))]
        );

        bus.write_int(8, 0x23, 1, Endianness::LittleEndian).unwrap();
        assert_eq!(bus.read_bytes(7, 3).unwrap(), [0x12, 0x23, 0x14]);

        bus.write_int(8, 0x24_23, 2, Endianness::LittleEndian).unwrap();
        assert_eq!(bus.read_bytes(7, 4).unwrap(), [0x12, 0x23, 0x24, 0x15]);
    }
}
