use std::collections::HashMap;

use crate::vm::Instruction;

#[derive(Debug, Clone)]
pub enum Item {
    Instruction(u8),
    PushLabel(String),
}

#[derive(Debug, Clone, Default)]
pub struct InstructionBuilder {
    items: Vec<Item>,
    labels: HashMap<String, u8>,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.labels.insert(label.into(), self.items.len() as _);
        self
    }

    pub fn just(mut self, instruction: Instruction) -> Self {
        self.items.push(Item::Instruction(instruction as u8));
        self
    }

    pub fn then<T: IntoPush>(mut self, instruction: Instruction, thing: T) -> Self {
        thing.into_push(&mut self);
        self.items.push(Item::Instruction(instruction as u8));
        self
    }

    pub fn push<T: IntoPush>(mut self, thing: T) -> Self {
        thing.into_push(&mut self);
        self
    }

    pub fn build(self) -> Vec<u8> {
        self.items
            .into_iter()
            .map(|item| match item {
                Item::Instruction(instruction) => instruction,
                Item::PushLabel(label) => self.labels[&label],
            })
            .collect()
    }
}

pub trait IntoPush: Sized {
    fn into_push(self, builder: &mut InstructionBuilder);
}

impl IntoPush for u8 {
    fn into_push(self, builder: &mut InstructionBuilder) {
        builder
            .items
            .push(Item::Instruction(Instruction::Push as u8));
        builder.items.push(Item::Instruction(self));
    }
}

impl IntoPush for &str {
    fn into_push(self, builder: &mut InstructionBuilder) {
        builder
            .items
            .push(Item::Instruction(Instruction::Push as u8));
        builder.items.push(Item::PushLabel(self.into()));
    }
}
