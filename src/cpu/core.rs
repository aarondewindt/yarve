use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display, Formatter, Debug};

use crate::bus::Bus;
use crate::endianness::Endianness;
use crate::device::{DeviceError, Device};
use crate::cpu::instruction::Instruction;
use crate::cpu::decode::InstructionDecodeError;
use crate::cpu::execute::InstructionExecuteError;
use crate::cpu::register::{XRegisterMap, FRegisterMap};


pub struct Core {
    pub pc: usize,
    pub x_registers: XRegisterMap,
    pub f_registers: FRegisterMap,
    pub bus: Rc<RefCell<Bus>>
}

#[derive(Debug)]
pub enum CoreError {
    DeviceError(DeviceError),
    InstructionDecodeError(InstructionDecodeError),
    InstructionExecuteError(InstructionExecuteError)
}

impl Core {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Core {
        Core {
            pc: 0,
            x_registers: XRegisterMap::new(),
            f_registers: FRegisterMap::new(),
            bus
        }
    }

    pub fn execute(&mut self) -> Result<(), CoreError>{
        let instruction = Instruction::decode(
            self.bus.borrow().read_int(
            self.pc,
            4,
            Endianness::LittleEndian,
                false
        )? as u32)?;

        instruction.execute(self)?;
        Ok(())
    }

    pub fn add_to_pc(&mut self, delta: i64) {
        self.pc = self.pc.wrapping_add(delta as usize);
    }

}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for CoreError {}

impl From<DeviceError> for CoreError {
    fn from(error: DeviceError) -> Self {
        Self::DeviceError(error)
    }
}

impl From<InstructionDecodeError> for CoreError {
    fn from(error: InstructionDecodeError) -> Self {
        Self::InstructionDecodeError(error)
    }
}

impl From<InstructionExecuteError> for CoreError {
    fn from(error: InstructionExecuteError) -> Self {
        Self::InstructionExecuteError(error)
    }
}
