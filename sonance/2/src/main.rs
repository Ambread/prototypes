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

    let mut memory = Memory::standard_io();
    let mut vm = VM::new(instructions);
    vm.add_device(&mut memory);
    vm.run()?;

    Ok(())
}
