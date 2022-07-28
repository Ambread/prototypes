use std::collections::HashMap;

use crate::vm::Instruction;

#[derive(Debug, Clone)]
pub enum BuildItem {
    Instruction(Instruction),
    PushLabel(String),
}

#[derive(Debug, Clone, Default)]
pub struct InstructionBuilder {
    instructions: Vec<BuildItem>,
    labels: HashMap<String, u64>,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.labels
            .insert(label.into(), self.instructions.len() as u64);
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

impl IntoPush for u64 {
    fn into_push(self) -> BuildItem {
        BuildItem::Instruction(Instruction::Push(self))
    }
}

impl IntoPush for &str {
    fn into_push(self) -> BuildItem {
        BuildItem::PushLabel(self.into())
    }
}
