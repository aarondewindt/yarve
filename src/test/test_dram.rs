
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
            match dram.write(address, value, 1, endianness) {
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
    }

    #[test]
    fn test_device_write_u16() {
        let mut dram = DRAM::new(16);

        let mut assert_case = |
                address: usize,
                value: u64,
                expected: [u8; 16],
                endianness: Endianness | {
            match dram.write(address, value, 2, endianness) {
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

        match dram.write(15, 19755, 2, Endianness::LittleEndian) {
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
            match dram.write(address, value, 4, endianness) {
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

        match dram.write(13, 19755, 4, Endianness::LittleEndian) {
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
            match dram.write(address, value, 8, endianness) {
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
            [0, 0, 0, 100, 0, 0, 0x1F, 0xE5, 0x1F, 0xE5, 0x3D, 0x7A, 0x2B, 0x4D, 0x32, 0x54],
            Endianness::LittleEndian
        );

        match dram.write(9, 0x54_32_4D_2B_7A_3D_E5_1F, 8, Endianness::LittleEndian) {
            Ok(()) => {panic!("PANIC")},
            Err(_) => {}
        }
    }

}
