use bf::{parse::parse, run::Memory, token::scan};
use clap::{AppSettings, Clap, FromArgMatches, IntoApp};
use std::{
    convert::TryInto,
    io::{self, Write},
};

fn main() {
    let mut buffer = String::new();
    let mut memory = Memory::new(100);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim();

        if input.starts_with('/') {
            let input = input[1..].split_whitespace();

            let input = Command::into_app()
                .setting(AppSettings::DisableHelpFlags)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::NoAutoHelp)
                .setting(AppSettings::NoBinaryName)
                .try_get_matches_from(input);

            buffer.clear();

            let input = match input {
                Ok(input) => Command::from_arg_matches(&input),
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };

            match input {
                Command::Get { location } => match location {
                    Location::Memory { index } => println!("{}", memory.get(index)),
                    Location::Current => println!("{}", memory.get(*memory.index())),
                    Location::Pointer => println!("{}", memory.index()),
                },
                Command::Set { value, location } => match location {
                    Location::Memory { index } => *memory.get(index) = value.try_into().unwrap(),
                    Location::Current => *memory.get(*memory.index()) = value.try_into().unwrap(),
                    Location::Pointer => *memory.index_mut() = value,
                },
                Command::Clear => memory.clear(),
                Command::Exit => break,
            }
            continue;
        }

        let input = parse(scan(input.chars()));

        memory.run(&input);

        buffer.clear();
    }
}

#[derive(Clap, Debug, Clone)]
enum Command {
    Get {
        #[clap(subcommand)]
        location: Location,
    },
    Set {
        value: usize,
        #[clap(subcommand)]
        location: Location,
    },
    Clear,
    Exit,
}

#[derive(Clap, Debug, Clone)]
enum Location {
    Memory { index: usize },
    Current,
    Pointer,
}
