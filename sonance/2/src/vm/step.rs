use crate::vm::{
    error::{Result, VMError},
    Instruction, VM,
};

impl VM {
    pub fn step(&mut self) -> Result<bool> {
        self.current_instruction = {
            let code = *self
                .instructions
                .get(self.instruction_index as usize)
                .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;

            let instruction: Result<Instruction, _> = code.try_into();
            instruction.map_err(|_| VMError::InvalidInstruction(code, self.instruction_index))?
        };

        // eprintln!(
        //     "{: >3}  {}",
        //     self.instruction_index, self.current_instruction,
        // );

        match self.current_instruction {
            Instruction::Halt => {
                return Ok(true);
            }
            Instruction::Debug => {
                eprintln!("{:?}", self.stack);
            }

            Instruction::Push => {
                self.instruction_index += 1;
                let value = *self
                    .instructions
                    .get(self.instruction_index as usize)
                    .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;
                self.stack.push(value);
            }
            Instruction::PushU16 => {
                for _ in 0..2 {
                    self.instruction_index += 1;
                    let value = *self
                        .instructions
                        .get(self.instruction_index as usize)
                        .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;
                    self.stack.push(value);
                }
            }
            Instruction::PushU32 => {
                for _ in 0..4 {
                    self.instruction_index += 1;
                    let value = *self
                        .instructions
                        .get(self.instruction_index as usize)
                        .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;
                    self.stack.push(value);
                }
            }
            Instruction::PushU64 => {
                for _ in 0..8 {
                    self.instruction_index += 1;
                    let value = *self
                        .instructions
                        .get(self.instruction_index as usize)
                        .ok_or(VMError::InstructionIndexOutOfBounds(self.instruction_index))?;
                    self.stack.push(value);
                }
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

                self.instruction_index = index;
            }
            Instruction::JumpIf => {
                let index = self.pop()?;
                let a = self.pop()?;
                if a == 0 {
                    self.instruction_index = index;
                }
            }

            Instruction::Load => {
                let variable = self.pop()?;
                let a = self.frames.load(variable).ok_or(VMError::ExpectedFrame(
                    self.current_instruction,
                    self.instruction_index,
                ))?;
                self.stack.push(a);
            }
            Instruction::Store => {
                let variable = self.pop()?;
                let a = self.pop()?;
                self.frames
                    .store(variable, a)
                    .ok_or(VMError::ExpectedFrame(
                        self.current_instruction,
                        self.instruction_index,
                    ))?;
            }

            Instruction::Read => {
                let device = self.pop()?;
                let index = self.pop_u32()?;
                let value = self.devices.read(device, index);
                self.stack.push(value)
            }
            Instruction::Write => {
                let device = self.pop()?;
                let index = self.pop_u32()?;
                let value = self.pop()?;
                self.devices.write(device, index, value);
            }
            Instruction::Resize => {
                let device = self.pop()?;

                let size = self.pop_u32()?;
                let value = self.pop()?;
                self.devices.resize(device, size, value);
            }
            Instruction::Flush => {
                let device = self.pop()?;
                let mode = self.pop()?;
                self.devices.flush(device, mode);
            }

            Instruction::Call => {
                let index = self.pop()?;
                self.frames.call(self.instruction_index);

                self.instruction_index = index;
            }
            Instruction::Return => {
                let index = self
                    .frames
                    .ret()
                    .ok_or(VMError::TopLevelReturn(self.instruction_index))?;

                self.instruction_index = index;
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

            Instruction::Eq => self.binary_op(|a, b| (a == b) as _)?,
            Instruction::Gt => self.binary_op(|a, b| (a > b) as _)?,
            Instruction::Geq => self.binary_op(|a, b| (a >= b) as _)?,
        }

        self.instruction_index += 1;

        Ok(false)
    }
}
