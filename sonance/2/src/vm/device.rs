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
}

/// A Device for storing data or performing IO
///
/// - 0: IO Register
///     - Read: Invalid
///     - Write:
///         - 0: Read stdin
///         - 1: Write stdout
///         - 2: Write stderr
///
/// - 1: Length Register
///     - Read: Get length
///     - Write: Set length (filling new empty space with 0)
///
/// - 2..9: Reserved
/// - 10..: Memory
///
#[derive(Debug, Clone, Default)]
pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    const IO_REGISTER: u32 = 0;
    const LEN_REGISTER: u32 = 1;
    const MEM_START: usize = 10;

    pub fn new() -> Self {
        Self::default()
    }
}

impl Device for Memory {
    fn read(&mut self, index: u32) -> u8 {
        if index == Self::IO_REGISTER {
            panic!("Cannot read from IO register (also todo add proper error handling)");
        }

        if index == Self::LEN_REGISTER {
            return self.memory.len() as u8;
        }

        self.memory[index as usize - Self::MEM_START]
    }

    fn write(&mut self, index: u32, value: u8) {
        if index == Self::IO_REGISTER {
            match value {
                0 => {
                    stdin().read_exact(&mut self.memory).unwrap();
                }
                1 => {
                    stdout().write_all(&self.memory).unwrap();
                }
                2 => {
                    stderr().write_all(&self.memory).unwrap();
                }
                _ => {}
            }
            return;
        }

        if index == Self::LEN_REGISTER {
            self.memory.resize(value as usize, 0);
            return;
        }

        self.memory[index as usize - Self::MEM_START] = value;
    }
}
