use num::{PrimInt};


pub fn extend_sign<T: PrimInt>(value: T, bits: usize) -> T {
    let sign_bit = (value >> (bits - 1usize)) & T::one();
    let mask = T::max_value() << bits;
    value | (sign_bit * mask)
}
