use std::collections::HashMap;

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

    pub fn ret(&mut self) -> Option<u8> {
        let frame = self.frames.pop()?;
        Some(frame.return_index)
    }

    pub fn load(&self, variable: u8) -> Option<u8> {
        let frame = self.frames.last()?;

        Some(frame.variables.get(&variable).copied().unwrap_or(0))
    }

    pub fn store(&mut self, variable: u8, value: u8) -> Option<()> {
        let frame = self.frames.last_mut()?;

        frame.variables.insert(variable, value);
        Some(())
    }
}
