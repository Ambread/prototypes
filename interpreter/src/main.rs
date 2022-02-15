use std::path::PathBuf;

use anyhow::Result;
use chumsky::Parser;
use clap::{AppSettings, ArgEnum, FromArgMatches, IntoApp, Parser as Clap};
use interpreter::{eval, parser};
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new();
    let mut current_mode = Mode::Eval;

    loop {
        let input = rl.readline(">> ");

        if let Err(ReadlineError::Interrupted | ReadlineError::Eof) = input {
            break;
        }

        let input = input?;
        rl.add_history_entry(input.as_str());

        if input.starts_with('/') {
            let command = match parse_command(&input) {
                Ok(command) => command,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };

            match command {
                Command::Exit => break,
                Command::Mode { mode } => current_mode = mode,
                Command::Load { file } => {
                    let input = std::fs::read_to_string(file)?;
                    interpret(&input, current_mode)
                }
            }
            continue;
        }

        interpret(&input, current_mode)
    }

    Ok(())
}

fn interpret(input: &str, current_mode: Mode) {
    let ast = match parser().parse(input) {
        Ok(ast) => ast,
        Err(parse_errs) => {
            parse_errs
                .into_iter()
                .for_each(|e| println!("Parse error: {}", e));
            return;
        }
    };

    if current_mode == Mode::Parse {
        println!("{ast:#?}");
        return;
    }

    let vars = &mut Vec::new();
    let funcs = &mut Vec::new();

    match eval(&ast, vars, funcs) {
        Ok(output) => println!("{:?}", output),
        Err(eval_err) => println!("Evaluation error: {}", eval_err),
    }
}

#[derive(Debug, Clone, Clap)]
enum Command {
    Exit,
    Mode {
        #[clap(arg_enum)]
        mode: Mode,
    },
    Load {
        file: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ArgEnum, PartialEq, Eq)]
enum Mode {
    Eval,
    Parse,
}

fn parse_command(input: &str) -> Result<Command> {
    let input = input.strip_prefix('/').unwrap().split_whitespace();

    let mut command = Command::into_app()
        .setting(AppSettings::DisableHelpFlag)
        .setting(AppSettings::DisableVersionFlag)
        .setting(AppSettings::NoAutoHelp)
        .setting(AppSettings::NoBinaryName)
        .setting(AppSettings::DisableHelpSubcommand);

    command.set_bin_name("/");
    let input = command.try_get_matches_from(input)?;

    Ok(Command::from_arg_matches(&input)?)
}
