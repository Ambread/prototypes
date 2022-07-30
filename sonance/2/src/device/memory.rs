use std::io::{stderr, stdin, stdout, Read, Write};

use crate::device::Device;

/// A Device for storing data or performing IO
///
/// - `1`: Perform command on slice
///     - Read: Result of previous command
///     - Write:
///         - `0`: Read all stdin
///         - `1`: Write all stdout
///         - `2`: Write all stderr
///         - `10`: Read any stdin
///         - `11`: Write any stdout
///         - `12`: Write any stderr
///         - `20`: Reset range
///
/// - `2`: Memory Length
/// - `3`: Slice Start
/// - `4`: Slice End
/// - `5`: Default Value
/// - `6..9`: Reserved
/// - `10..`: Memory
///
#[derive(Default)]
pub struct Memory {
    pub memory: Vec<u8>,
    run_command: Option<Box<dyn FnMut(&mut Memory, u8) -> u8>>,
    io_result: u8,
    io_start: u8,
    io_end: u8,
    fill_value: u8,
}

mod register {
    pub const COMMAND: u32 = 1;
    pub const LENGTH: u32 = 2;
    pub const SLICE_START: u32 = 3;
    pub const SLICE_END: u32 = 4;
    pub const DEFAULT_VALUE: u32 = 5;
    pub const MEMORY: usize = 10;
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_command_mock<F>(run_command: F) -> Self
    where
        F: FnMut(&mut Memory, u8) -> u8 + 'static,
    {
        Self {
            run_command: Some(Box::new(run_command)),
            ..Default::default()
        }
    }

    pub fn io_slice(&self) -> &[u8] {
        &self.memory[self.io_start as usize..self.io_end as usize]
    }

    pub fn io_slice_mut(&mut self) -> &mut [u8] {
        &mut self.memory[self.io_start as usize..self.io_end as usize]
    }

    fn run_standard_command(&mut self, instruction: u8) -> u8 {
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
            0 => todo!(),

            register::COMMAND => self.io_result,
            register::LENGTH => self.memory.len() as u8,
            register::SLICE_START => self.io_start,
            register::SLICE_END => self.io_end,
            register::DEFAULT_VALUE => self.fill_value,

            _ => self.memory[index as usize - register::MEMORY],
        }
    }

    fn write(&mut self, index: u32, value: u8) {
        match index {
            0 => todo!(),

            register::COMMAND => {
                if let Some(mut run_command) = self.run_command.take() {
                    self.io_result = run_command(self, value);
                    self.run_command = Some(run_command);
                } else {
                    self.io_result = self.run_standard_command(value);
                }
            }

            register::LENGTH => self.memory.resize(value as usize, self.fill_value),
            register::SLICE_START => self.io_start = value,
            register::SLICE_END => self.io_end = value,
            register::DEFAULT_VALUE => self.fill_value = value,

            _ => self.memory[index as usize - register::MEMORY] = value,
        }
    }
}
