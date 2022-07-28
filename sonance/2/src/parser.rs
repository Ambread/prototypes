use std::collections::HashMap;

use crate::vm::Instruction;

#[derive(Debug, Clone)]
enum Item {
    Instruction(Instruction),
    PushLabel(String),
}

impl From<Instruction> for Item {
    fn from(instruction: Instruction) -> Self {
        Self::Instruction(instruction)
    }
}

#[derive(Debug, Clone, Default)]
struct InstructionParser {
    items: Vec<Item>,
    labels: HashMap<String, u8>,
}

pub fn parse(src: &str) -> Vec<Instruction> {
    let mut parser = InstructionParser::default();

    for line in src.lines() {
        parser.parse_line(line);
    }

    parser.build()
}

impl InstructionParser {
    fn parse_line(&mut self, line: &str) {
        let line = line.trim();

        if line.is_empty() {
            return;
        }

        if let Some(label) = self.parse_label(line) {
            self.labels.insert(label, self.items.len() as _);
            return;
        }

        let mut args = line.split_whitespace();
        let instruction = args.next().unwrap();

        for arg in args.rev() {
            let item = self.parse_arg(arg);
            self.items.push(item);
        }

        if instruction != "push" {
            let instruction: Instruction = instruction.parse().unwrap();
            self.items.push(instruction.into());
        }
    }

    fn parse_arg(&mut self, arg: &str) -> Item {
        if let Some(label) = self.parse_label(arg) {
            return Item::PushLabel(label);
        }

        let number = arg.parse().unwrap();
        Instruction::Push(number).into()
    }

    fn parse_label(&mut self, thing: &str) -> Option<String> {
        thing
            .starts_with('#')
            .then(|| thing.trim_start_matches('#').into())
    }

    fn build(self) -> Vec<Instruction> {
        self.items
            .into_iter()
            .map(|item| match item {
                Item::Instruction(instruction) => instruction,
                Item::PushLabel(label) => {
                    let value = self.labels[&label];
                    Instruction::Push(value)
                }
            })
            .collect()
    }
}
