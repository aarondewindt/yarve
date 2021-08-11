use crate::device::{Device, DeviceError};
use crate::endianness::Endianness;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::any::Any;


pub struct DRAM {
    size: usize,
    memory: Vec<u8>
}

impl DRAM {
    pub fn new(size: usize) -> DRAM {
        Self {
            size,
            memory: vec![0; size + 7]
        }
    }
}

impl Debug for DRAM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DRAM {{ size: {:?} }}", self.size)
    }
}

impl Device for DRAM {
    fn get_address_space_size(&self) -> usize { self.size }

    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], DeviceError> {
        let end_address = address + size;

        if end_address <= self.size {
            Ok(&self.memory[address..(address+size)])
        } else {
            Err(DeviceError::InvalidAddress)
        }
    }

    fn write_bytes(&mut self, address: usize, binary: &[u8]) -> Result<(), DeviceError> {
        let end_address = address + binary.len();

        if end_address <= self.size {
            self.memory.splice(
                address..end_address,
                binary.iter().cloned());
            Ok(())

        } else {
            Err(DeviceError::InvalidAddress)
        }
    }

    fn read_int(&self, address: usize, size: usize, endianness: Endianness) -> Result<u64, DeviceError> {
        if (address + size) <= self.size {
            match size {
                1..=8 => {
                    let bytes = &self.memory[address..(address+8)];

                    let bytes = match bytes.try_into() {
                        Ok(i) => i,
                        Err(e) => {
                            return Err(DeviceError::InternalDeviceError(Box::new(e)))
                        }
                    };

                    match endianness {
                        Endianness::LittleEndian => {
                            let mask = u64::MAX >> ((8 - size) * 8);
                            Ok(u64::from_le_bytes(bytes) & mask)
                        },
                        Endianness::BigEndian => {
                            let mask = u64::MAX << ((8 - size) * 8);
                            Ok((u64::from_be_bytes(bytes) & mask) >> ((8 - size) * 8))
                        }
                    }

                },
                _ => Err(DeviceError::InvalidSize)
            }
        } else {
            Err(DeviceError::InvalidAddress)
        }
    }

    fn write_int(&mut self, address: usize, value: u64, size: usize, endianness: Endianness)
                 -> Result<(), DeviceError> {
        if (address + size) <= self.size {
            match size {
                1..=8 => {
                    match endianness {
                        Endianness::LittleEndian => {
                            self.write_bytes(address, &value.to_le_bytes()[0..size])
                        },
                        Endianness::BigEndian => {
                            self.write_bytes(address, &value.to_be_bytes()[(8-size)..])
                        }
                    }
                }
                _ => Err(DeviceError::InvalidSize)
            }
        } else {
            Err(DeviceError::InvalidAddress)
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}
