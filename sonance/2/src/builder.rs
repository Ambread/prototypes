use std::{collections::HashMap, str::FromStr};

use crate::vm::Instruction;

#[derive(Debug, Clone)]
pub enum BuildItem {
    Instruction(Instruction),
    PushLabel(String),
}

#[derive(Debug, Clone, Default)]
pub struct InstructionBuilder {
    instructions: Vec<BuildItem>,
    labels: HashMap<String, u8>,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.labels
            .insert(label.into(), self.instructions.len() as _);
        self
    }

    pub fn just(mut self, instruction: Instruction) -> Self {
        self.instructions.push(BuildItem::Instruction(instruction));
        self
    }

    pub fn then<T: IntoPush>(mut self, instruction: Instruction, thing: T) -> Self {
        self.instructions.push(thing.into_push());
        self.instructions.push(BuildItem::Instruction(instruction));
        self
    }

    pub fn push<T: IntoPush>(mut self, thing: T) -> Self {
        self.instructions.push(thing.into_push());
        self
    }

    pub fn build(self) -> Vec<Instruction> {
        self.instructions
            .into_iter()
            .map(|item| match item {
                BuildItem::Instruction(instruction) => instruction,
                BuildItem::PushLabel(label) => {
                    let value = self.labels[&label];
                    Instruction::Push(value)
                }
            })
            .collect()
    }
}

pub trait IntoPush: Sized {
    fn into_push(self) -> BuildItem;
}

impl IntoPush for u8 {
    fn into_push(self) -> BuildItem {
        BuildItem::Instruction(Instruction::Push(self))
    }
}

impl IntoPush for &str {
    fn into_push(self) -> BuildItem {
        BuildItem::PushLabel(self.into())
    }
}

impl FromStr for InstructionBuilder {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut builder = Self::new();

        for line in src.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.ends_with(':') {
                builder = builder.label(line.trim_end_matches(':'));
                continue;
            }

            let mut args = line.split_whitespace();
            let instruction = args.next().unwrap();

            for arg in args.rev() {
                match arg.parse::<u8>() {
                    Ok(number) => builder = builder.push(number),
                    Err(_) => builder = builder.push(arg),
                }
            }

            if instruction != "push" {
                let instruction = instruction.parse::<Instruction>().unwrap();
                builder = builder.just(instruction);
            }
        }

        Ok(builder)
    }
}
