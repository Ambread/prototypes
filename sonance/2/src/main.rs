use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use sonance::{
    parser,
    vm::{Memory, VM},
};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.input)?;
    let instructions = parser::parse(&src)?;

    let mut vm = VM::new(instructions);
    vm.attach(Memory::new());
    vm.run()?;

    Ok(())
}
