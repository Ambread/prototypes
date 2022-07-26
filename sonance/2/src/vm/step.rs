use crate::{
    error::Result,
    vm::{Instruction, VM},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Normal,
    Jump,
    Halt,
}

impl VM {
    pub fn step(&mut self) -> Result<Step> {
        if self.is_halted {
            return Ok(Step::Halt);
        }

        self.current_instruction = self.instructions[self.instruction_index];
        match self.current_instruction {
            Instruction::Halt => {
                self.is_halted = true;
                return Ok(Step::Halt);
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
                return Ok(Step::Jump);
            }
            Instruction::JumpIf(index) => {
                let a = self.pop()?;
                if a != 0 {
                    self.instruction_index = index;
                    return Ok(Step::Jump);
                }
            }

            Instruction::Load(variable) => {
                let a = self.frames.load(variable)?;
                self.stack.push(a);
            }
            Instruction::Store(variable) => {
                let a = self.pop()?;
                self.frames.store(variable, a)?;
            }

            Instruction::Call(index) => {
                self.frames.call(self.instruction_index + 1);
                self.instruction_index = index;
                return Ok(Step::Jump);
            }
            Instruction::Return => {
                self.instruction_index = self.frames.ret()?;
                return Ok(Step::Jump);
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

        Ok(Step::Normal)
    }
}
