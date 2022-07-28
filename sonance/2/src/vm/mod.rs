pub mod error;
mod frames;
mod instruction;
mod step;

use crate::vm::error::{Result, VMError};
pub use crate::vm::{
    frames::{Frame, Frames},
    instruction::Instruction,
};

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    pub instructions: Vec<Instruction>,
    pub instruction_index: u64,
    pub current_instruction: Instruction,
    pub has_jumped: bool,
    pub stack: Vec<u64>,
    pub frames: Frames,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            instructions: vec![Instruction::Halt],
            instruction_index: 0,
            current_instruction: Instruction::Halt,
            has_jumped: false,
            stack: vec![],
            frames: Default::default(),
        }
    }
}

impl VM {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.step()? {}
        Ok(())
    }

    fn pop(&mut self) -> Result<u64> {
        self.stack
            .pop()
            .ok_or(VMError::EmptyStack(self.current_instruction))
    }

    fn jump(&mut self, index: u64) {
        self.instruction_index = index;
        self.has_jumped = true;
    }

    fn unary_op(&mut self, body: impl FnOnce(u64) -> u64) -> Result<()> {
        let a = self.pop()?;
        self.stack.push(body(a));
        Ok(())
    }

    fn binary_op<F>(&mut self, body: F) -> Result<()>
    where
        F: FnOnce(u64, u64) -> u64,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(body(a, b));
        Ok(())
    }
}
