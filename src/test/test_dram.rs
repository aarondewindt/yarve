
#[cfg(test)]
mod test_dram {
    use crate::dram::DRAM;
    use crate::device::Device;
    use crate::endianness::Endianness;

    #[test]
    fn test_write_read_slice_full_memory() {
        let mut dram = DRAM::new(16);

        let buffer: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        match dram.write_bytes(0, &buffer) {
            Ok(()) => {},
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_bytes(0, 16) {
            Ok(read_buffer) => { assert_eq!(buffer, read_buffer)},
            Err(e) => {panic!("PANIC {:?}", e)}
        }
    }

    #[test]
    fn test_write_read_slice_partial_memory() {
        let mut dram = DRAM::new(16);

        let write_buffer: [u8; 5] = [1, 2, 3, 4, 5];
        let expected_read: [u8; 9] = [0, 0, 1, 2, 3, 4, 5, 0, 0];

        match dram.write_bytes(7, &write_buffer) {
            Ok(()) => {},
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_bytes(5, 9) {
            Ok(read_buffer) => { assert_eq!(expected_read, read_buffer)},
            Err(e) => {panic!("PANIC {:?}", e)}
        }
    }

    #[test]
    fn test_device_write_u8() {
        let mut dram = DRAM::new(16);

        let mut assert_case = |
                address: usize,
                value: u64,
                expected: [u8; 16],
                endianness: Endianness | {
            match dram.write_int(address, value, 1, endianness) {
                Ok(()) => {},
                Err(e) => {panic!("PANIC {:?}", e)}
            }

            match dram.read_bytes(0, 16) {
                Ok(read_buffer) => { assert_eq!(expected, read_buffer)},
                Err(e) => {panic!("PANIC {:?}", e)}
            }
        };

        assert_case(
            3,
            100,
            [0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            4,
            50,
            [0, 0, 0, 100, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            2,
            25,
            [0, 0, 25, 100, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            1,
            51256,
            [0, 56, 25, 100, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            5,
            51256,
            [0, 56, 25, 100, 50, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            15,
            12,
            [0, 56, 25, 100, 50, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12],
            Endianness::LittleEndian
        );

        assert_case(
            1,
            24,
            [0, 24, 25, 100, 50, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12],
            Endianness::BigEndian
        );
    }

    #[test]
    fn test_device_write_u16() {
        let mut dram = DRAM::new(16);

        let mut assert_case = |
                address: usize,
                value: u64,
                expected: [u8; 16],
                endianness: Endianness | {
            match dram.write_int(address, value, 2, endianness) {
                Ok(()) => {},
                Err(e) => {panic!("PANIC {:?}", e)}
            }

            match dram.read_bytes(0, 16) {
                Ok(read_buffer) => { assert_eq!(expected, read_buffer)},
                Err(e) => {panic!("PANIC {:?}", e)}
            }
        };

        assert_case(
            3,
            100,
            [0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            7,
            26936,
            [0, 0, 0, 100, 0, 0, 0, 56, 105, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            2,
            19755,
            [0, 0, 43, 77, 0, 0, 0, 56, 105, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            0,
            19755,
            [43, 77, 43, 77, 0, 0, 0, 56, 105, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            13,
            0x2a_08,
            [43, 77, 43, 77, 0, 0, 0, 56, 105, 0, 0, 0, 0, 0x2a, 0x08, 0],
            Endianness::BigEndian
        );

        match dram.write_int(15, 19755, 2, Endianness::LittleEndian) {
            Ok(()) => {panic!("PANIC")},
            Err(_) => {}
        }

    }

    #[test]
    fn test_device_write_u32() {
        let mut dram = DRAM::new(16);

        let mut assert_case = |
                address: usize,
                value: u64,
                expected: [u8; 16],
                endianness: Endianness | {
            match dram.write_int(address, value, 4, endianness) {
                Ok(()) => {},
                Err(e) => { panic!("PANIC {:?}", e) }
            }

            match dram.read_bytes(0, 16) {
                Ok(read_buffer) => { assert_eq!(expected, read_buffer) },
                Err(e) => { panic!("PANIC {:?}", e) }
            }
        };

        assert_case(
            3,
            100,
            [0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            6,
            0x54_32_4D_2B,
            [0, 0, 0, 100, 0, 0, 0x2B, 0x4D, 0x32, 0x54, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            12,
            0x7A_3D_E5_1F,
            [0, 0, 0, 100, 0, 0, 0x2B, 0x4D, 0x32, 0x54, 0, 0, 0x1F, 0xE5, 0x3D, 0x7A],
            Endianness::LittleEndian
        );

        assert_case(
            12,
            0x7A_3D_E5_1F,
            [0, 0, 0, 100, 0, 0, 0x2B, 0x4D, 0x32, 0x54, 0, 0, 0x7A, 0x3D, 0xE5, 0x1F],
            Endianness::BigEndian
        );

        match dram.write_int(13, 19755, 4, Endianness::LittleEndian) {
            Ok(()) => {panic!("PANIC")},
            Err(_) => {}
        }

    }

    #[test]
    fn test_device_write_u64() {
        let mut dram = DRAM::new(16);

        let mut assert_case = |
                address: usize,
                value: u64,
                expected: [u8; 16],
                endianness: Endianness | {
            match dram.write_int(address, value, 8, endianness) {
                Ok(()) => {},
                Err(e) => { panic!("PANIC {:?}", e) }
            }

            match dram.read_bytes(0, 16) {
                Ok(read_buffer) => { assert_eq!(expected, read_buffer) },
                Err(e) => { panic!("PANIC {:?}", e) }
            }
        };

        assert_case(
            3,
            100,
            [0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            6,
            0x54_32_4D_2B_7A_3D_E5_1F,
            [0, 0, 0, 100, 0, 0, 0x1F, 0xE5, 0x3D, 0x7A, 0x2B, 0x4D, 0x32, 0x54, 0, 0],
            Endianness::LittleEndian
        );

        assert_case(
            8,
            0x54_32_4D_2B_7A_3D_E5_1F,
            [
                0, 0, 0, 100, 0, 0, 0x1F, 0xE5,
                0x1F, 0xE5, 0x3D, 0x7A, 0x2B, 0x4D, 0x32, 0x54],
            Endianness::LittleEndian
        );

        assert_case(
            4,
            0x54_32_4D_2B_7A_3D_E5_1F,
            [
                0, 0, 0, 100, 0x54, 0x32, 0x4D, 0x2B,
                0x7A, 0x3D, 0xE5, 0x1F, 0x2B, 0x4D, 0x32, 0x54],
            Endianness::BigEndian
        );

        match dram.write_int(9, 0x54_32_4D_2B_7A_3D_E5_1F, 8, Endianness::LittleEndian) {
            Ok(()) => {panic!("PANIC")},
            Err(_) => {}
        }
    }

    #[test]
    fn test_device_read_u8() {
        let mut dram = DRAM::new(16);

        let buffer = [
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];

        match dram.write_bytes(0, &buffer) {
            Ok(()) => {},
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(2, 1, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x12),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(2, 1, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x12),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(15, 1, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x1F),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(15, 1, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x1F),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(0, 1, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x10),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(0, 1, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x10),
            Err(e) => {panic!("PANIC {:?}", e)}
        }
    }

    #[test]
    fn test_device_read_u16() {
        let mut dram = DRAM::new(16);

        let buffer = [
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];

        match dram.write_bytes(0, &buffer) {
            Ok(()) => {},
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(2, 2, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x13_12),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(2, 2, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x12_13),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(14, 2, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x1F_1E),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(14, 2, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x1E_1F),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(0, 2, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x11_10),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(0, 2, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x10_11),
            Err(e) => {panic!("PANIC {:?}", e)}
        }

        match dram.read_int(15, 2, Endianness::BigEndian) {
            Ok(value) => {
                println!("{:x}", value);
            },
            Err(_) => {}
        }
    }

    #[test]
    fn test_device_read_u32() {
        let mut dram = DRAM::new(16);

        let buffer = [
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];

        match dram.write_bytes(0, &buffer) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 4, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x15_14_13_12),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 4, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x12_13_14_15),
            Err(e) => { panic!("PANIC {:?}", e) }
        }
    }

    #[test]
    fn test_device_read_u64() {
        let mut dram = DRAM::new(16);

        let buffer = [
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];

        match dram.write_bytes(0, &buffer) {
            Ok(()) => {},
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 8, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x19_18_17_16_15_14_13_12),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 8, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x12_13_14_15_16_17_18_19),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 8, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x19_18_17_16_15_14_13_12),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(2, 8, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x12_13_14_15_16_17_18_19),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(8, 8, Endianness::LittleEndian) {
            Ok(value) => assert_eq!(value, 0x1F_1E_1D_1C_1B_1A_19_18),
            Err(e) => { panic!("PANIC {:?}", e) }
        }

        match dram.read_int(8, 8, Endianness::BigEndian) {
            Ok(value) => assert_eq!(value, 0x18_19_1A_1B_1C_1D_1E_1F),
            Err(e) => { panic!("PANIC {:?}", e) }
        }
    }

}
