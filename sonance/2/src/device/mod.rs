pub mod memory;

use std::fmt::Debug;

pub trait Device {
    fn read(&mut self, index: u32) -> u8;
    fn write(&mut self, index: u32, value: u8);
}

#[derive(Default)]
pub struct DeviceManager<'a> {
    devices: Vec<&'a mut dyn Device>,
    selected: u8,
}

impl<'a> DeviceManager<'a> {
    pub fn add<T: Device + 'a>(&mut self, device: &'a mut T) {
        self.devices.push(device);
    }

    pub fn read(&mut self, index: u32) -> u8 {
        if index == 0 {
            return self.selected;
        }

        self.devices[self.selected as usize].read(index)
    }

    pub fn write(&mut self, index: u32, value: u8) {
        if index == 0 {
            return self.selected = value;
        }

        self.devices[self.selected as usize].write(index, value);
    }
}

impl Debug for DeviceManager<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DebugWrapper")
            .field("devices", &self.devices.len())
            .finish()
    }
}

impl Clone for DeviceManager<'_> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for DeviceManager<'_> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
