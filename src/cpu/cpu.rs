use crate::cpu::core::Core;
use crate::device::Device;
use crate::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;


struct _CPU {
    core: Core,
    bus: Rc<RefCell<Bus>>
}

impl _CPU {
    fn _new(devices: Vec<(usize, Box<dyn Device>)>) -> Self {
        let bus = Rc::new(RefCell::new(Bus::new(devices)));
        Self {
            core: Core::new(bus.clone()),
            bus
        }
    }
}
