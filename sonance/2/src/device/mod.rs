pub mod memory;

use std::fmt::Debug;

pub trait Device {
    fn read(&mut self, index: u32) -> u8;
    fn write(&mut self, index: u32, value: u8);
}

#[derive(Default)]
pub struct DeviceManager {
    devices: Vec<Box<dyn Device>>,
    selected: usize,
}

impl DeviceManager {
    pub fn attach<T: Device + 'static>(&mut self, device: T) {
        self.devices.push(Box::new(device));
    }

    pub fn select(&mut self, device: u8) {
        self.selected = device as usize;
    }

    pub fn read(&mut self, index: u32) -> u8 {
        self.devices[self.selected].read(index)
    }

    pub fn write(&mut self, index: u32, value: u8) {
        self.devices[self.selected].write(index, value);
    }
}

impl Debug for DeviceManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebugWrapper")
            .field("devices", &self.devices.len())
            .finish()
    }
}

impl Clone for DeviceManager {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for DeviceManager {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
