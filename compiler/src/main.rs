use anyhow::Result;
use std::{fs, process::Command};

use compiler::compile;

fn main() -> Result<()> {
    let input = fs::read_to_string("wew.son")?;
    let output = compile(&input);
    fs::write("wew.ssa", output)?;

    Command::new("qbe")
        .args(["wew.ssa", "-o", "wew.s"])
        .output()?;

    Command::new("cc").args(["wew.s", "-o", "wew"]).output()?;

    Ok(())
}
