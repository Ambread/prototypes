use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use sonance::{device::memory::Memory, parser, vm::VM};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.input)?;
    let instructions = parser::parse(&src)?;

    let mut vm = VM::new(instructions).with_device(Memory::new());
    vm.run()?;

    Ok(())
}
