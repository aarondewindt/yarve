use std::error::Error;
use crate::endianness::Endianness;
use std::fmt::{Display, Formatter, Debug};
use std::any::Any;

#[derive(Debug)]
pub enum DeviceError {
    InvalidAddressWriteFault,
    InvalidAddressReadFault,
    InvalidSizeWriteFault,
    InvalidSizeReadFault,
    InternalDeviceError(Box<dyn Error>),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DeviceError {}

pub trait Device: Debug {
    fn get_address_space_size(&self) -> usize;
    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], DeviceError>;
    fn write_bytes(&mut self, address: usize, binary: &[u8]) -> Result<(), DeviceError>;
    fn read_int(&self, address: usize, size: usize, endianness: Endianness, sign_extend: bool)
        -> Result<u64, DeviceError>;
    fn write_int(&mut self, address: usize, value: u64, size: usize, endianness: Endianness)
                 -> Result<(), DeviceError>;
    fn as_any(&self) -> &dyn Any;
}
