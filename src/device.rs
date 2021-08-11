use std::error::Error;
use crate::endianness::Endianness;

#[derive(Debug)]
pub enum DeviceError {
    InvalidAddress,
    InvalidSize,
    InternalDeviceError(Box<dyn Error>)
}

pub trait Device {
    fn get_address_space_size(&self) -> usize;
    fn read(&self, address: usize, size: usize, endianness: Endianness) -> Result<u64, DeviceError>;
    fn write(&mut self, address: usize, value: u64, size: usize, endianness: Endianness)
        -> Result<(), DeviceError>;
}
