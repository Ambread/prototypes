use std::io::{stderr, stdin, stdout, Read, Write};

use crate::device::Device;

/// A Device for storing data or performing IO
///
/// - `0`: Perform IO
///     - Read: Result of previous IO
///     - Write:
///         - `0`: Read all stdin
///         - `1`: Write all stdout
///         - `2`: Write all stderr
///         - `10`: Read any stdin
///         - `11`: Write any stdout
///         - `12`: Write any stderr
///         - `20`: Reset IO range
///
/// - `1`: Memory Length
/// - `2`: IO Slice Start
/// - `3`: IO Slice End
/// - `4..9`: Reserved
/// - `10..`: Memory
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Memory {
    pub memory: Vec<u8>,
    io_result: u8,
    io_start: u8,
    io_end: u8,
    fill_value: u8,
}

mod register {
    pub const IO: u32 = 0;
    pub const LEN: u32 = 1;
    pub const IO_START: u32 = 2;
    pub const IO_END: u32 = 3;
    pub const FILL_VALUE: u32 = 4;
    pub const MEMORY: usize = 10;
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn io_slice(&self) -> &[u8] {
        &self.memory[self.io_start as usize..self.io_end as usize]
    }

    pub fn io_slice_mut(&mut self) -> &mut [u8] {
        &mut self.memory[self.io_start as usize..self.io_end as usize]
    }

    fn perform_io(&mut self, instruction: u8) -> u8 {
        (match instruction {
            0 => {
                stdin().read_exact(self.io_slice_mut()).unwrap();
                self.memory.len()
            }
            1 => {
                stdout().write_all(self.io_slice()).unwrap();
                self.memory.len()
            }
            3 => {
                stderr().write_all(self.io_slice()).unwrap();
                self.memory.len()
            }

            10 => stdin().read(self.io_slice_mut()).unwrap(),
            11 => stdout().write(self.io_slice()).unwrap(),
            13 => stderr().write(self.io_slice()).unwrap(),

            20 => {
                let fill_value = self.fill_value;
                self.io_slice_mut().fill(fill_value);
                self.io_slice_mut().len()
            }

            _ => return self.io_result,
        }) as u8
    }
}

impl Device for Memory {
    fn read(&mut self, index: u32) -> u8 {
        match index {
            register::IO => self.io_result,
            register::LEN => self.memory.len() as u8,
            register::IO_START => self.io_start,
            register::IO_END => self.io_end,
            register::FILL_VALUE => self.fill_value,

            _ => self.memory[index as usize - register::MEMORY],
        }
    }

    fn write(&mut self, index: u32, value: u8) {
        match index {
            register::IO => self.io_result = self.perform_io(value),
            register::LEN => self.memory.resize(value as usize, self.fill_value),
            register::IO_START => self.io_start = value,
            register::IO_END => self.io_end = value,
            register::FILL_VALUE => self.fill_value = value,

            _ => self.memory[index as usize - register::MEMORY] = value,
        }
    }
}
