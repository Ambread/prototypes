use std::io::Write;

use console::Term;

use crate::vm::{
    error::{Result, VMError},
    Instruction, VM,
};

impl VM {
    pub fn step(&mut self) -> Result<bool> {
        self.current_instruction = *self
            .instructions
            .get(self.instruction_index as usize)
            .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;

        match self.current_instruction {
            Instruction::Halt => {
                return Ok(true);
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

            Instruction::Jump => {
                let index = self.pop()?;
                self.jump(index);
            }
            Instruction::JumpIf => {
                let index = self.pop()?;
                let a = self.pop()?;
                if a == 0 {
                    self.jump(index);
                }
            }

            Instruction::Load => {
                let variable = self.pop()?;
                let a = self.frames.load(variable)?;
                self.stack.push(a);
            }
            Instruction::Store => {
                let variable = self.pop()?;
                let a = self.pop()?;
                self.frames.store(variable, a)?;
            }

            Instruction::Call => {
                let index = self.pop()?;
                self.frames.call(self.instruction_index + 1);
                self.jump(index);
            }
            Instruction::Return => {
                let index = self.frames.ret()?;
                self.jump(index);
            }

            Instruction::In => {
                let char = Term::stdout().read_char().unwrap();
                self.stack.push(char as u64);
            }
            Instruction::Out => {
                let buffer = self.pop()?.to_ne_bytes();
                std::io::stdout().write_all(&buffer).unwrap();
                std::io::stdout().flush().unwrap();
            }

            Instruction::Add => self.binary_op(|a, b| a + b)?,
            Instruction::Sub => self.binary_op(|a, b| a - b)?,
            Instruction::Mul => self.binary_op(|a, b| a * b)?,
            Instruction::Div => self.binary_op(|a, b| a / b)?,

            Instruction::BitAnd => self.binary_op(|a, b| a & b)?,
            Instruction::BitOr => self.binary_op(|a, b| a | b)?,
            Instruction::BitNot => self.unary_op(|a| !a)?,

            Instruction::BoolAnd => self.binary_op(|a, b| (a != 0 && b != 0) as _)?,
            Instruction::BoolOr => self.binary_op(|a, b| (a != 0 || b != 0) as _)?,
            Instruction::BoolNot => self.unary_op(|a| (a == 0) as _)?,

            Instruction::Eq => self.binary_op(|a, b| (a == b) as u64)?,
            Instruction::Gt => self.binary_op(|a, b| (a > b) as u64)?,
            Instruction::Geq => self.binary_op(|a, b| (a >= b) as u64)?,
        }

        // Avoid incrementing if a instruction jumped to avoid off-by-one situations
        if !self.has_jumped {
            self.instruction_index += 1;
        }
        self.has_jumped = false;

        Ok(false)
    }
}
