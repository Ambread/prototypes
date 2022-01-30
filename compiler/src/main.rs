use anyhow::Result;
use clap::Parser;
use std::{fs, path::PathBuf, process::Command};

use compiler::compile;

#[derive(Debug, Parser)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = fs::read_to_string(&args.input)?;
    let output = compile(&input);
    fs::write(args.input.with_extension(".ssa"), output)?;

    Command::new("qbe")
        .arg(args.input.with_extension(".ssa"))
        .arg("-o")
        .arg(args.input.with_extension(".s"))
        .output()?;

    Command::new("cc")
        .arg(args.input.with_extension(".s"))
        .arg("-o")
        .arg(args.input.file_stem().unwrap())
        .output()?;

    Ok(())
}
