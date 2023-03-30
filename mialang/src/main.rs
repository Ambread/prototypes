mod ext;
mod token;

use std::io::stdin;

use itertools::Itertools;

use crate::{ext::Spanned, token::Lexer};

fn main() {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let source = buffer.trim();

        let tokens = Lexer::new(source).collect_vec();

        for Spanned(span, token) in tokens {
            print!(
                "{}{}{} {:?} ",
                " ".repeat(span.start),
                "-".repeat(span.end - span.start + 1),
                " ".repeat(source.len() - span.end),
                span,
            );

            match token {
                Ok(token) => println!("{token:?}"),
                Err(token) => println!("{token}"),
            }
        }
        println!();
    }
}
