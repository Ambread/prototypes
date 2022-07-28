use std::collections::HashMap;

use crate::vm::Instruction;

#[derive(Debug, Clone)]
enum Item {
    Instruction(u8),
    PushLabel(String),
}

#[derive(Debug, Clone, Default)]
struct InstructionParser {
    items: Vec<Item>,
    labels: HashMap<String, u8>,
}

pub fn parse(src: &str) -> Vec<u8> {
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
            self.parse_arg(arg);
        }

        if instruction != "push" {
            let instruction: Instruction = instruction.parse().unwrap();
            self.items.push(Item::Instruction(instruction as u8));
        }
    }

    fn parse_arg(&mut self, arg: &str) {
        self.items.push(Item::Instruction(Instruction::Push as u8));

        if let Some(label) = self.parse_label(arg) {
            self.items.push(Item::PushLabel(label));
            return;
        }

        let number = arg.parse().unwrap();
        self.items.push(Item::Instruction(number));
    }

    fn parse_label(&mut self, thing: &str) -> Option<String> {
        thing
            .starts_with('#')
            .then(|| thing.trim_start_matches('#').into())
    }

    fn build(self) -> Vec<u8> {
        self.items
            .into_iter()
            .map(|item| match item {
                Item::Instruction(instruction) => instruction,
                Item::PushLabel(label) => self.labels[&label],
            })
            .collect()
    }
}
