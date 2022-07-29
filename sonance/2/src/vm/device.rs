use std::io::{stderr, stdout, Read, Write};
use std::{fmt::Debug, io::stdin};

#[derive(Default)]
pub struct DeviceManager {
    devices: Vec<Box<dyn Device>>,
}

impl DeviceManager {
    pub fn attach<T: Device + 'static>(&mut self, device: T) {
        self.devices.push(Box::new(device));
    }

    pub fn read(&mut self, device: u8, index: u32) -> u8 {
        self.devices[device as usize].read(index)
    }

    pub fn write(&mut self, device: u8, index: u32, value: u8) {
        self.devices[device as usize].write(index, value);
    }

    pub fn resize(&mut self, device: u8, size: u32, value: u8) {
        self.devices[device as usize].resize(size, value);
    }

    pub fn flush(&mut self, device: u8, mode: u8) {
        self.devices[device as usize].flush(mode);
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

pub trait Device {
    fn read(&mut self, index: u32) -> u8;
    fn write(&mut self, index: u32, value: u8);
    fn resize(&mut self, size: u32, value: u8);
    fn flush(&mut self, mode: u8);
}

impl Device for Vec<u8> {
    fn read(&mut self, index: u32) -> u8 {
        self[index as usize]
    }

    fn write(&mut self, index: u32, value: u8) {
        self[index as usize] = value;
    }

    fn resize(&mut self, size: u32, value: u8) {
        self.resize(size as usize, value);
    }

    fn flush(&mut self, mode: u8) {
        match mode {
            0 => {
                stdin().read_exact(self).unwrap();
            }
            1 => {
                stdout().write_all(self).unwrap();
            }
            2 => {
                stderr().write_all(self).unwrap();
            }
            _ => {}
        }
    }
}
