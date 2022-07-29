use std::io::{stdin, stdout, Read, Write};

use crate::device::Device;

/// A Device for storing data or performing IO
///
/// - 0: IO Register
///     - Read: Result of previous IO
///     - Write:
///         - 0: Read all stdin
///         - 1: Write all stdout
///         - 10: Read any stdin
///         - 11: Write any stdout
///
/// - 1: Length Register
///     - Read: Get length
///     - Write: Set length (filling new empty space with 0)
///
/// - 2..9: Reserved
/// - 10..: Memory
///
#[derive(Default)]
pub struct Memory {
    pub memory: Vec<u8>,
    io_result: u8,
    perform_io: Option<Box<dyn FnMut(&mut Memory, u8) -> u8>>,
}

impl Memory {
    const IO_REGISTER: u32 = 0;
    const LEN_REGISTER: u32 = 1;
    const MEM_START: usize = 10;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_io_mock<F>(perform_io: F) -> Self
    where
        F: FnMut(&mut Memory, u8) -> u8 + 'static,
    {
        Self {
            perform_io: Some(Box::new(perform_io)),
            ..Default::default()
        }
    }

    fn standard_perform_io(&mut self, instruction: u8) -> u8 {
        (match instruction {
            0 => {
                stdin().read_exact(&mut self.memory).unwrap();
                self.memory.len()
            }
            1 => {
                stdout().write_all(&self.memory).unwrap();
                self.memory.len()
            }

            10 => stdin().read(&mut self.memory).unwrap(),
            11 => stdout().write(&self.memory).unwrap(),

            _ => return self.io_result,
        }) as u8
    }
}

impl Device for Memory {
    fn read(&mut self, index: u32) -> u8 {
        if index == Self::IO_REGISTER {
            return self.io_result;
        }

        if index == Self::LEN_REGISTER {
            return self.memory.len() as u8;
        }

        self.memory[index as usize - Self::MEM_START]
    }

    fn write(&mut self, index: u32, value: u8) {
        if index == Self::IO_REGISTER {
            if let Some(mut perform_io) = self.perform_io.take() {
                self.io_result = perform_io(self, value);
                self.perform_io = Some(perform_io);
                return;
            }

            self.io_result = self.standard_perform_io(value);
            return;
        }

        if index == Self::LEN_REGISTER {
            self.memory.resize(value as usize, 0);
            return;
        }

        self.memory[index as usize - Self::MEM_START] = value;
    }
}
