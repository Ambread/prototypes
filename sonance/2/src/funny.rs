use crate::vm::Instruction;

#[derive(Debug, Clone, Default)]
pub struct BuilderToParseBuilder {
    output: String,
}

impl BuilderToParseBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label(mut self, label: &str) -> Self {
        self.output += "#";
        self.output += label;
        self.output += "\n";
        self
    }

    pub fn just(mut self, instruction: Instruction) -> Self {
        self.output += &instruction.to_string();
        self.output += "\n";
        self
    }

    pub fn then<T: IntoPush>(mut self, instruction: Instruction, thing: T) -> Self {
        self.output += &instruction.to_string();
        self.output += " ";
        thing.into_push(&mut self);
        self.output += "\n";
        self
    }

    pub fn push<T: IntoPush>(mut self, thing: T) -> Self {
        self.output += "push ";
        thing.into_push(&mut self);
        self.output += "\n";
        self
    }

    pub fn build(self) -> Vec<u8> {
        println!("{}", self.output);
        vec![]
    }
}

pub trait IntoPush: Sized {
    fn into_push(self, builder: &mut BuilderToParseBuilder);
}

impl IntoPush for u8 {
    fn into_push(self, builder: &mut BuilderToParseBuilder) {
        builder.output += &self.to_string();
    }
}

impl IntoPush for &str {
    fn into_push(self, builder: &mut BuilderToParseBuilder) {
        builder.output += "#";
        builder.output += self;
    }
}
