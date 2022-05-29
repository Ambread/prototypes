mod generate;
mod lexer;
mod parser;
#[cfg(test)]
mod test;

use std::{fs, path::PathBuf, process::Command};

use anyhow::Result;
use chumsky::Parser;
use clap::Parser as Clap;

use generate::generate;
use lexer::lexer;
use parser::parser;

#[derive(Debug, Clap)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = fs::read_to_string(&args.input)?;

    let tokens = lexer().parse(input).unwrap();
    fs::write(
        args.input.with_extension("token.json"),
        serde_json::to_string_pretty(&tokens)?,
    )?;

    let ast = parser().parse(tokens).unwrap();
    fs::write(
        args.input.with_extension("ast.json"),
        serde_json::to_string_pretty(&ast)?,
    )?;

    let output = generate(ast);
    fs::write(args.input.with_extension("ssa"), output)?;

    Command::new("qbe")
        .arg(args.input.with_extension("ssa"))
        .arg("-o")
        .arg(args.input.with_extension("s"))
        .output()?;

    Command::new("cc")
        .arg(args.input.with_extension("s"))
        .arg("-o")
        .arg(args.input.with_extension(""))
        .output()?;

    let exit_code = Command::new(args.input.with_extension(""))
        .output()?
        .status
        .code();

    println!("exit_code: {exit_code:?}");

    Ok(())
}
