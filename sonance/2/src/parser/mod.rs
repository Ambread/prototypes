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

    parser.build()
}

impl InstructionParser {
    fn parse_line(&mut self, line: &str) -> Result<()> {
        let line = line
            .split("//")
            .next()
            .expect("split should always give at least one str")
            .trim();

        if line.is_empty() {
            return Ok(());
        }

        if let Some(label) = self.parse_prefixed("#", line) {
            self.labels.insert(label, self.items.len() as _);
            return Ok(());
        }

        let mut args = line.split_whitespace();
        let instruction_str = args
            .next()
            .expect("split should always give at least one str");

        for arg in args.rev() {
            self.parse_arg(arg)?;
        }

        let instruction = instruction_str.parse()?;
        if !instruction_str.starts_with("push") {
            self.items.push(Item::Instruction(instruction));
        }

        Ok(())
    }

    fn parse_arg(&mut self, arg: &str) -> Result<()> {
        if let Some(label) = self.parse_prefixed("#", arg) {
            self.items.push(Item::Instruction(Instruction::Push));
            self.items.push(Item::LabelReference(label));
            return Ok(());
        }

        if let Some(variable) = self.parse_prefixed("&", arg) {
            let variable = *self.variables.entry(variable).or_insert_with(|| {
                let variable = self.variable_counter;
                self.variable_counter += 1;
                variable
            });

            self.items.push(Item::Instruction(Instruction::Push));
            self.items.push(Item::Raw(variable));
            return Ok(());
        }

        if let Some(char) = self.parse_wrapped("'", arg) {
            self.items.push(Item::Instruction(Instruction::Push));

            let char = match char.as_str() {
                "\\n" => b'\n',
                char => char.chars().next().unwrap_or(' ') as u8,
            };

            self.items.push(Item::Raw(char));
            return Ok(());
        }

        let arg = &arg.replace('_', "");

        if let Some(number) = self.parse_suffixed(arg, "u8") {
            let number = number.parse()?;
            self.items.push(Item::Instruction(Instruction::Push));
            self.items.push(Item::Raw(number));
            return Ok(());
        }

        if let Some(number) = self.parse_suffixed(arg, "u16") {
            let number: u16 = number.parse()?;
            self.items.push(Item::Instruction(Instruction::PushU16));
            for raw in number.to_le_bytes() {
                self.items.push(Item::Raw(raw));
            }
            return Ok(());
        }

        if let Some(number) = self.parse_suffixed(arg, "u32") {
            let number: u32 = number.parse()?;
            self.items.push(Item::Instruction(Instruction::PushU32));
            for raw in number.to_le_bytes() {
                self.items.push(Item::Raw(raw));
            }
            return Ok(());
        }

        if let Some(number) = self.parse_suffixed(arg, "u64") {
            let number: u64 = number.parse()?;
            self.items.push(Item::Instruction(Instruction::PushU64));
            for raw in number.to_le_bytes() {
                self.items.push(Item::Raw(raw));
            }
            return Ok(());
        }

        let number = arg.parse()?;
        self.items.push(Item::Instruction(Instruction::Push));
        self.items.push(Item::Raw(number));

        Ok(())
    }

    fn parse_prefixed(&mut self, prefix: &str, ident: &str) -> Option<String> {
        ident
            .starts_with(prefix)
            .then(|| ident.trim_start_matches(prefix).into())
    }

    fn parse_suffixed(&mut self, ident: &str, suffix: &str) -> Option<String> {
        ident
            .ends_with(suffix)
            .then(|| ident.trim_end_matches(suffix).into())
    }

    fn parse_wrapped(&mut self, delimiter: &str, ident: &str) -> Option<String> {
        (ident.starts_with(delimiter) && ident.ends_with(delimiter)).then(|| {
            ident
                .trim_start_matches(delimiter)
                .trim_end_matches(delimiter)
                .into()
        })
    }

    fn build(self) -> Result<Vec<u8>> {
        self.items
            .into_iter()
            .map(|item| match item {
                Item::Instruction(instruction) => Ok(instruction as u8),
                Item::Raw(raw) => Ok(raw),
                Item::LabelReference(label) => self
                    .labels
                    .get(&label)
                    .map(|index| index - 1)
                    .ok_or(ParseError::LabelNotFound(label)),
            })
            .collect()
    }
}
