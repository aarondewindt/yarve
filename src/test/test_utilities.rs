

#[cfg(test)]
mod test_utilities {
    use crate::utilities::extend_sign;

    #[test]
    fn test_extend_sign_u16() {
        let value = extend_sign(0b10000000u16, 8);
        assert_eq_hex!(value, 0b11111111_10000000);
        let value = extend_sign(0b01000000u16, 8);
        assert_eq_hex!(value, 0b00000000_01000000);
    }

    #[test]
    fn test_extend_sign_u64() {
        let value = extend_sign(0b10000000u64, 8);
        assert_eq_hex!(value, 0xffffffffffffff_80);
        let value = extend_sign(0b01000000u64, 8);
        assert_eq_hex!(value, 0x00000000000000_40);
    }
}