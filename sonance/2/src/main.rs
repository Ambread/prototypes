use std::collections::HashMap;

#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Default)]
pub struct VM {
    instructions: Vec<Instruction>,
    instruction_index: usize,
    is_halted: bool,
    stack: Vec<usize>,
    frames: Vec<Frame>,
}

#[derive(Debug, Clone, Default)]
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
            frames: vec![Default::default()],
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        while self.step().unwrap() {}
    }

    pub fn step(&mut self) -> Option<bool> {
        if self.is_halted {
            return Some(false);
        }

        match self.instructions[self.instruction_index] {
            Instruction::Halt => {
                self.is_halted = true;
            }

            Instruction::Push(value) => {
                self.stack.push(value);
            }
            Instruction::Pop => {
                self.stack.pop()?;
            }
            Instruction::Dupe => {
                let a = *self.stack.last()?;
                self.stack.push(a);
            }

            Instruction::Jump(index) => {
                self.instruction_index = index;
                return Some(true); // Don't ++ index at end
            }
            Instruction::JumpIf(index) => {
                let a = self.stack.pop()?;
                if a != 0 {
                    self.instruction_index = index;
                    return Some(true); // Don't ++ index at end
                }
            }

            Instruction::Load(variable) => {
                let a = self
                    .frames
                    .last()?
                    .variables
                    .get(&variable)
                    .copied()
                    .unwrap_or_default();
                self.stack.push(a);
            }
            Instruction::Store(variable) => {
                let a = self.stack.pop()?;
                self.frames.last_mut()?.variables.insert(variable, a);
            }

            Instruction::Call(index) => {
                self.frames.push(Frame::new(self.instruction_index + 1));
                self.instruction_index = index;
                return Some(true); // Don't ++ index at end
            }
            Instruction::Return => {
                self.instruction_index = self.frames.pop()?.return_index;
                return Some(true); // Don't ++ index at end
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
        Some(true)
    }

    fn unary_op(&mut self, body: impl FnOnce(usize) -> usize) -> Option<()> {
        let a = self.stack.pop()?;
        self.stack.push(body(a));
        Some(())
    }

    fn binary_op<F>(&mut self, body: F) -> Option<()>
    where
        F: FnOnce(usize, usize) -> usize,
    {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(body(a, b));
        Some(())
    }
}
