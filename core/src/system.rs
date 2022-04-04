use crate::cpu::Cpu;
use crate::device::{Device, IoPortHandler};
use crate::mem::MemMap;

pub struct System<C>
where
    C: Cpu + ?Sized,
{
    pub cpu: Box<C>,
    pub mem: MemMap,
    pub devices: Vec<Box<dyn Device>>,
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn new(cpu: C, mem: MemMap) -> Self {
        Self {
            cpu: Box::new(cpu),
            mem,
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: impl Device + 'static) {
        self.devices.push(Box::new(device));
    }

    pub fn init(&mut self) {
        for device in &mut self.devices {
            device.init();
        }
        self.cpu.init();
    }

    pub fn start(&mut self) {
        self.cpu.reset();
        loop {
            for device in &mut self.devices {
                device.step();
            }
            C::step(self);
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.start();
    }
}

macro_rules! port_handler {
    ($self:ident, $port:ident, $variant:ident) => {
        $self
            .devices
            .iter_mut()
            .filter_map(|device| {
                device
                    .ports()
                    .remove(&$port)
                    .map(|handlers| (device, handlers))
            })
            .filter_map(|(device, handlers)| {
                let handler = handlers
                    .iter()
                    .filter_map(|handler| match handler {
                        IoPortHandler::$variant(handler) => Some(handler),
                        _ => None,
                    })
                    .next();

                handler.map(|handler| (&mut **device, *handler))
            })
            .next()
    };
}

impl<C> System<C>
where
    C: Cpu,
{
    pub fn port_in_8(&mut self, port: u16) -> Option<u8> {
        port_handler!(self, port, In8).map(|(device, handler)| handler(device))
    }

    pub fn port_in_16(&mut self, port: u16) -> Option<u16> {
        port_handler!(self, port, In16).map(|(device, handler)| handler(device))
    }

    pub fn port_out_8(&mut self, port: u16, value: u8) {
        port_handler!(self, port, Out8).map(|(device, handler)| handler(device, value));
    }

    pub fn port_out_16(&mut self, port: u16, value: u16) {
        port_handler!(self, port, Out16).map(|(device, handler)| handler(device, value));
    }
}
