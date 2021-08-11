// use crate::device::{Device, Error as DeviceError};
//
//
// pub struct Bus {
//     base_address: u64,
//     end_address: u64,
//     devices: Vec<dyn Device>
// }
//
// impl Bus {
//     fn new(base_address: u64, devices: Vec<dyn Device>) -> Bus {
//         let mut end_address: u64 = 0;
//
//         for device in devices {
//             let (_, device_end_address) = device.get_address_space();
//
//             if (base_address + device_end_address) > end_address {
//                 end_address = base_address + device_end_address;
//             }
//         }
//
//         Bus {
//             base_address,
//             end_address,
//             devices
//         }
//     }
// }
//
// impl Device for Bus {
//     fn get_address_space(&self) -> (u64, u64){
//         (self.base_address, self.end_address)
//     }
//
//     fn read(&self, address: u64, size: u8) -> Result<u64, DeviceError> {
//         match address {
//             base_address..=end_address => {
//
//             },
//             _ => Err(DeviceError::InvalidAddress)
//         }
//     }
//
//     fn write(&mut self, address: u64, value: u64, size: u8) -> Result<u64, DeviceError> {
//
//     }
// }