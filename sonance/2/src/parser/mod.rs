mod error;

use std::collections::HashMap;

use crate::instruction::Instruction;
pub use crate::parser::error::{ParseError, Result};

#[derive(Debug, Clone)]
enum Item {
    Instruction(Instruction),
    Raw(u8),
    LabelReference(String),
}

#[derive(Debug, Clone, Default)]
struct InstructionParser {
    items: Vec<Item>,
    labels: HashMap<String, u8>,
    variables: HashMap<String, u8>,
    variable_counter: u8,
}

pub fn parse(src: &str) -> Result<Vec<u8>> {
    let mut parser = InstructionParser::default();

    for line in src.lines() {
        parser.parse_line(line)?;
    }

    Ok(parser.build())
}

impl InstructionParser {
    fn parse_line(&mut self, line: &str) -> Result<()> {
        let line = line
            .split(';')
            .next()
            .expect("split should always give at least one str")
            .trim();

        if line.is_empty() {
            return Ok(());
        }

        if let Some(label) = self.parse_prefixed('#', line) {
            self.labels.insert(label, self.items.len() as _);
            return Ok(());
        }

        let mut args = line.split_whitespace();
        let instruction = args
            .next()
            .expect("split should always give at least one str");

        for arg in args.rev() {
            self.parse_arg(arg)?;
        }

        let instruction = instruction.parse()?;
        if instruction != Instruction::Push {
            self.items.push(Item::Instruction(instruction));
        }

        Ok(())
    }

    fn parse_arg(&mut self, arg: &str) -> Result<()> {
        self.items.push(Item::Instruction(Instruction::Push));

        if let Some(label) = self.parse_prefixed('#', arg) {
            self.items.push(Item::LabelReference(label));
            return Ok(());
        }

        if let Some(variable) = self.parse_prefixed('&', arg) {
            let variable = *self.variables.entry(variable).or_insert_with(|| {
                let variable = self.variable_counter;
                self.variable_counter += 1;
                variable
            });

            self.items.push(Item::Raw(variable));
            return Ok(());
        }

        let number = arg.parse()?;
        self.items.push(Item::Raw(number));

        Ok(())
    }

    fn parse_prefixed(&mut self, prefix: char, ident: &str) -> Option<String> {
        ident
            .starts_with(prefix)
            .then(|| ident.trim_start_matches(prefix).into())
    }

    fn build(self) -> Vec<u8> {
        self.items
            .into_iter()
            .map(|item| match item {
                Item::Instruction(instruction) => instruction as u8,
                Item::Raw(raw) => raw,
                Item::LabelReference(label) => self.labels[&label] - 1,
            })
            .collect()
    }
}
