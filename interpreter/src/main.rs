use anyhow::Result;
use chumsky::Parser;
use interpreter::{eval, parser};
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline(">> ");

        if let Err(ReadlineError::Interrupted | ReadlineError::Eof) = line {
            break;
        }

        let line = line?;
        rl.add_history_entry(line.as_str());

        let ast = match parser().parse(line) {
            Ok(ast) => ast,
            Err(parse_errs) => {
                parse_errs
                    .into_iter()
                    .for_each(|e| println!("Parse error: {}", e));
                continue;
            }
        };

        let vars = &mut Vec::new();
        let funcs = &mut Vec::new();

        match eval(&ast, vars, funcs) {
            Ok(output) => println!("{}", output),
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        }
    }

    Ok(())
}
