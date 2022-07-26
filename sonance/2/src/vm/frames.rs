use std::collections::HashMap;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Frame {
    pub variables: HashMap<usize, usize>,
    pub return_index: usize,
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

    pub fn call(&mut self, return_index: usize) {
        self.frames.push(Frame {
            return_index,
            ..Default::default()
        });
    }

    pub fn ret(&mut self) -> Result<usize> {
        let frame = self.frames.pop().ok_or(Error::TopLevelReturn)?;
        Ok(frame.return_index)
    }

    pub fn load(&self, variable: usize) -> Result<usize> {
        let frame = self.frames.last().ok_or(Error::ExpectedFrame)?;

        Ok(frame.variables.get(&variable).copied().unwrap_or(0))
    }

    pub fn store(&mut self, variable: usize, value: usize) -> Result<()> {
        let frame = self.frames.last_mut().ok_or(Error::ExpectedFrame)?;

        frame.variables.insert(variable, value);
        Ok(())
    }
}
