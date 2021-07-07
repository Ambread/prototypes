mod codegen;

use anyhow::Result;
use std::{env::current_dir, fs};

fn main() -> Result<()> {
    let dir = current_dir()?.join("dev");

    let input = fs::read_to_string(dir.join("input.son"))?.trim().parse()?;

    fs::write(
        dir.join("output.o"),
        codegen::codegen("low_level_lang", input)?,
    )?;

    Ok(())
}
