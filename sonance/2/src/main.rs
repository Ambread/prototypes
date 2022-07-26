use std::collections::HashMap;

use thiserror::Error;

#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Halt,

    Push(usize),
    Pop,
    Dupe,

    Jump(usize),
    JumpIf(usize),

    Load(usize),
    Store(usize),

    Call(usize),
    Return,

    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    Not,

    Eq,
    Gt,
    Geq,
}

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("instruction {0:?} wanted a value from the stack, but it was empty")]
    EmptyStack(Instruction),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    instructions: Vec<Instruction>,
    instruction_index: usize,
    current_instruction: Instruction,
    is_halted: bool,
    stack: Vec<usize>,
    frames: Vec<Frame>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            instructions: vec![Instruction::Halt],
            instruction_index: 0,
            current_instruction: Instruction::Halt,
            is_halted: true,
            stack: vec![],
            frames: vec![Default::default()],
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Frame {
    variables: HashMap<usize, usize>,
    return_index: usize,
}

impl Frame {
    fn new(return_index: usize) -> Self {
        Self {
            return_index,
            ..Default::default()
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

    pub fn run(&mut self) {
        while self.step().unwrap() {}
    }

    pub fn step(&mut self) -> Result<bool> {
        if self.is_halted {
            return Ok(false);
        }

        self.current_instruction = self.instructions[self.instruction_index];
        match self.current_instruction {
            Instruction::Halt => {
                self.is_halted = true;
            }

            Instruction::Push(value) => {
                self.stack.push(value);
            }
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::Dupe => {
                let a = self.pop()?;
                self.stack.push(a);
                self.stack.push(a);
            }

            Instruction::Jump(index) => {
                self.instruction_index = index;
                return Ok(true); // Don't ++ index at end
            }
            Instruction::JumpIf(index) => {
                let a = self.pop()?;
                if a != 0 {
                    self.instruction_index = index;
                    return Ok(true); // Don't ++ index at end
                }
            }

            Instruction::Load(variable) => {
                let a = self
                    .frames
                    .last()
                    .unwrap()
                    .variables
                    .get(&variable)
                    .copied()
                    .unwrap_or_default();
                self.stack.push(a);
            }
            Instruction::Store(variable) => {
                let a = self.pop()?;
                self.frames
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(variable, a);
            }

            Instruction::Call(index) => {
                self.frames.push(Frame::new(self.instruction_index + 1));
                self.instruction_index = index;
                return Ok(true); // Don't ++ index at end
            }
            Instruction::Return => {
                self.instruction_index = self.frames.pop().unwrap().return_index;
                return Ok(true); // Don't ++ index at end
            }

            Instruction::Add => self.binary_op(|a, b| a + b)?,
            Instruction::Sub => self.binary_op(|a, b| a - b)?,
            Instruction::Mul => self.binary_op(|a, b| a * b)?,
            Instruction::Div => self.binary_op(|a, b| a / b)?,

            Instruction::And => self.binary_op(|a, b| a & b)?,
            Instruction::Or => self.binary_op(|a, b| a | b)?,
            Instruction::Not => self.unary_op(|a| if a == 0 { 1 } else { 0 })?,

            Instruction::Eq => self.binary_op(|a, b| (a == b) as usize)?,
            Instruction::Gt => self.binary_op(|a, b| (a > b) as usize)?,
            Instruction::Geq => self.binary_op(|a, b| (a >= b) as usize)?,
        }

        self.instruction_index += 1;

        Ok(true)
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
