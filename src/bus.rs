use crate::device::{Device, DeviceError};
use crate::endianness::Endianness;
use rangemap::RangeMap;
use std::any::Any;
use core::ops::Range;


#[derive(Debug)]
pub struct Bus {
    address_space_size: usize,
    address_space_map: RangeMap<usize, usize>,
    devices: Vec<Box<dyn Device>>
}

impl Bus {
    pub fn new(devices: Vec<(usize, Box<dyn Device>)>) -> Self {
        let mut bus = Self {
            address_space_size: 0,
            address_space_map: RangeMap::new(),
            devices: Vec::new()
        };

        for (base_address, device) in devices {
            let address_space_size = device.get_address_space_size();
            let end_address = base_address + address_space_size;

            if bus.address_space_size < end_address {
                bus.address_space_size = end_address;
            }

            bus.address_space_map.insert(base_address..end_address, bus.devices.len());
            bus.devices.push(device)
        }

        bus
    }

    pub fn get_device(&self, address: usize) -> Option<(&Range<usize>, &Box<dyn Device>)> {
        let (address_range, device_idx) =
            match self.address_space_map.get_key_value(&address) {
                Some(x) => x,
                None => { return None }
        };
        Some((address_range, &self.devices[*device_idx]))
    }

    pub fn get_device_mut(&mut self, address: usize) -> Option<(&Range<usize>, &mut Box<dyn Device>)> {
        let (address_range, device_idx) =
            match self.address_space_map.get_key_value(&address) {
                Some(x) => x,
                None => { return None }
            };

        Some((address_range, match self.devices.get_mut(*device_idx) {
            Some(x) => x,
            None => { return None }
        }))
    }
}

impl Device for Bus {
    fn get_address_space_size(&self) -> usize{ self.address_space_size }

    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], DeviceError> {
        match self.get_device(address) {
            Some((address_range, device)) => {
                let address = address - address_range.start;
                device.read_bytes(address, size)
            },
            None => { Err(DeviceError::InvalidAddressReadFault) }
        }
    }

    fn write_bytes(&mut self, address: usize, binary: &[u8]) -> Result<(), DeviceError> {
        match self.get_device_mut(address) {
            Some((address_range, device)) => {
                let address = address - address_range.start;
                device.write_bytes(address, binary)
            },
            None => { Err(DeviceError::InvalidAddressWriteFault) }
        }
    }

    fn read_int(&self, address: usize, size: usize, endianness: Endianness) -> Result<u64, DeviceError> {
        match self.get_device(address) {
            Some((address_range, device)) => {
                let address = address - address_range.start;
                device.read_int(address, size, endianness)
            },
            None => { Err(DeviceError::InvalidAddressReadFault) }
        }
    }

    fn write_int(&mut self, address: usize, value: u64, size: usize, endianness: Endianness) -> Result<(), DeviceError> {
        match self.get_device_mut(address) {
            Some((address_range, device)) => {
                let address = address - address_range.start;
                device.write_int(address, value, size, endianness)
            },
            None => { Err(DeviceError::InvalidAddressWriteFault) }
        }
    }

    fn as_any(&self) -> &dyn Any { self }
}