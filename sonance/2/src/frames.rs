use std::collections::HashMap;

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

    pub fn ret(&mut self) -> usize {
        self.frames.pop().unwrap().return_index
    }

    fn last(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn last_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn load(&self, variable: usize) -> usize {
        self.last().variables.get(&variable).copied().unwrap_or(0)
    }

    pub fn store(&mut self, variable: usize, value: usize) {
        self.last_mut().variables.insert(variable, value);
    }
}
