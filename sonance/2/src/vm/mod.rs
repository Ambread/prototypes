mod frames;
mod instruction;
mod step;

use crate::error::{Error, Result};
pub use crate::vm::{
    frames::{Frame, Frames},
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    pub instructions: Vec<Instruction>,
    pub instruction_index: usize,
    pub current_instruction: Instruction,
    pub is_halted: bool,
    pub stack: Vec<usize>,
    pub frames: Frames,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            instructions: vec![Instruction::Halt],
            instruction_index: 0,
            current_instruction: Instruction::Halt,
            is_halted: true,
            stack: vec![],
            frames: Default::default(),
        }
    }
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            is_halted: false,
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }

    fn pop(&mut self) -> Result<usize> {
        self.stack
            .pop()
            .ok_or(Error::EmptyStack(self.current_instruction))
    }

    fn unary_op(&mut self, body: impl FnOnce(usize) -> usize) -> Result<()> {
        let a = self.pop()?;
        self.stack.push(body(a));
        Ok(())
    }

    fn binary_op<F>(&mut self, body: F) -> Result<()>
    where
        F: FnOnce(usize, usize) -> usize,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(body(a, b));
        Ok(())
    }
}
