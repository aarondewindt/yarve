use crate::device::{Device, DeviceError};
use crate::endianness::Endianness;
use std::any::Any;
use std::fmt::{Formatter, Debug};

const _UART_RBR: u8 = 0;
const UART_THR: u8 = 0;
const _UART_DLL: u8 = 0;
const _UART_IER: u8 = 1;
const _UART_DLM: u8 = 1;
const _UART_IIR: u8 = 2;
const _UART_FCR: u8 = 2;
const _UART_LCR: u8 = 3;
const _UART_MCR: u8 = 4;
const _UART_LSR: u8 = 5;
const _UART_MSR: u8 = 6;
const _UART_SCR: u8 = 7;

pub struct UART {

}

impl UART {
    fn new() -> Self {
        Self { }
    }
}

impl Debug for UART {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UART")
    }
}

impl Device for UART {
    fn get_address_space_size(&self) -> usize { 8 }

    fn read_bytes(&self, _address: usize, size: usize) -> Result<&[u8], DeviceError> {
        if size == 1 {
            Ok(&[0])
        }
        else {
            Err(DeviceError::InvalidSizeReadFault)
        }
    }

    fn write_bytes(&mut self, address: usize, binary: &[u8]) -> Result<(), DeviceError> {
        if binary.len() == 1 {
            match address as u8 {
                UART_THR => print!("{}", binary[0] as char),
                _ => {}
            }
            Ok(())
        }
        else {
            Err(DeviceError::InvalidSizeWriteFault)
        }
    }

    fn read_int(&self, address: usize, size: usize, _endianness: Endianness) -> Result<u64, DeviceError> {
        Ok(self.read_bytes(address, size)?[0] as u64)
    }

    fn write_int(&mut self, address: usize, value: u64, size: usize, _endianness: Endianness) -> Result<(), DeviceError> {
        if size == 1 {
            Ok(self.write_bytes(address, &[value as u8])?)
        } else {
            Err(DeviceError::InvalidSizeWriteFault)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
