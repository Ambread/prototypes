use std::collections::HashMap;

use crate::vm::error::{Result, VMError};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Frame {
    pub variables: HashMap<u8, u8>,
    pub return_index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frames {
    frames: Vec<Frame>,
}

impl Default for Frames {
    fn default() -> Self {
        Self {
            frames: vec![Default::default()],
        }
    }
}

impl Frames {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self { frames }
    }

    pub fn call(&mut self, return_index: u8) {
        self.frames.push(Frame {
            return_index,
            ..Default::default()
        });
    }

    pub fn ret(&mut self) -> Result<u8> {
        let frame = self.frames.pop().ok_or(VMError::TopLevelReturn)?;
        Ok(frame.return_index)
    }

    pub fn load(&self, variable: u8) -> Result<u8> {
        let frame = self.frames.last().ok_or(VMError::ExpectedFrame)?;

        Ok(frame.variables.get(&variable).copied().unwrap_or(0))
    }

    pub fn store(&mut self, variable: u8, value: u8) -> Result<()> {
        let frame = self.frames.last_mut().ok_or(VMError::ExpectedFrame)?;

        frame.variables.insert(variable, value);
        Ok(())
    }
}
