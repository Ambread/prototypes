use std::io::{Read, Write};

use crate::device::Device;

/// A Device for storing data or performing IO
///
/// - `1`: Perform command on slice
///     - Read: Result of previous command
///     - Write:
///         - `0`: Read from input
///         - `1`: Read all from input
///         - `2`: Write to output
///         - `3`: Write all to output
///         - `4`: Reset
///
/// - `2`: Memory Length
/// - `3`: Slice Start
/// - `4`: Slice End
/// - `5`: Default Value
/// - `6`: IO Index
/// - `7..9`: Reserved
/// - `10..`: Memory
///
#[derive(Default)]
pub struct Memory<'a> {
    pub memory: Vec<u8>,

    inputs: Option<Vec<Box<dyn Read + 'a>>>,
    outputs: Option<Vec<Box<dyn Write + 'a>>>,

    command_result: u8,
    slice_start: u8,
    slice_end: u8,
    default_value: u8,
    io_index: u8,
}

mod register {
    pub const COMMAND: u32 = 1;
    pub const LENGTH: u32 = 2;
    pub const SLICE_START: u32 = 3;
    pub const SLICE_END: u32 = 4;
    pub const DEFAULT_VALUE: u32 = 5;
    pub const IO_INDEX: u32 = 6;
    pub const MEMORY: usize = 10;
}

impl<'a> Memory<'a> {
    pub fn empty_io() -> Self {
        Self::default()
    }

    pub fn standard_io() -> Self {
        let mut memory = Self::default();
        memory.add_input(std::io::stdin());
        memory.add_output(std::io::stdout());
        memory.add_output(std::io::stderr());
        memory
    }

    pub fn add_input<T: Read + 'a>(&mut self, input: T) {
        let mut inputs = self.inputs.take().unwrap_or_default();
        inputs.push(Box::new(input));
        self.inputs = Some(inputs);
    }

    pub fn add_output<T: Write + 'a>(&mut self, output: T) {
        let mut outputs = self.outputs.take().unwrap_or_default();
        outputs.push(Box::new(output));
        self.outputs = Some(outputs);
    }

    pub fn slice(&self) -> &[u8] {
        &self.memory[self.slice_start as usize..self.slice_end as usize]
    }

    pub fn slice_mut(&mut self) -> &mut [u8] {
        &mut self.memory[self.slice_start as usize..self.slice_end as usize]
    }

    fn run_command(&mut self, instruction: u8) {
        let io_index = self.io_index as usize;
        let mut inputs = self.inputs.take().unwrap_or_default();
        let mut outputs = self.outputs.take().unwrap_or_default();

        self.command_result = (match instruction {
            0 => inputs[io_index].read(self.slice_mut()).unwrap(),
            1 => {
                inputs[io_index].read_exact(self.slice_mut()).unwrap();
                self.memory.len()
            }

            2 => outputs[io_index].write(self.slice()).unwrap(),
            3 => {
                outputs[io_index].write_all(self.slice()).unwrap();
                self.memory.len()
            }

            4 => {
                let fill_value = self.default_value;
                self.slice_mut().fill(fill_value);
                self.slice_mut().len()
            }

            _ => self.command_result as usize,
        }) as u8;

        self.inputs = Some(inputs);
        self.outputs = Some(outputs);
    }
}

impl<'a> Device for Memory<'a> {
    fn read(&mut self, index: u32) -> u8 {
        match index {
            0 => todo!(),

            register::COMMAND => self.command_result,
            register::LENGTH => self.memory.len() as u8,
            register::SLICE_START => self.slice_start,
            register::SLICE_END => self.slice_end,
            register::DEFAULT_VALUE => self.default_value,
            register::IO_INDEX => self.io_index,

            _ => self.memory[index as usize - register::MEMORY],
        }
    }

    fn write(&mut self, index: u32, value: u8) {
        match index {
            0 => todo!(),

            register::COMMAND => self.run_command(value),
            register::LENGTH => self.memory.resize(value as usize, self.default_value),
            register::SLICE_START => self.slice_start = value,
            register::SLICE_END => self.slice_end = value,
            register::DEFAULT_VALUE => self.default_value = value,
            register::IO_INDEX => self.io_index = value,

            _ => self.memory[index as usize - register::MEMORY] = value,
        }
    }
}
