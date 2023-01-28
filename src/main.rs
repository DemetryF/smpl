use std::{env, fs};

use crate::{static_analyzer::static_error::StaticErrorKind, translator::Translator};

mod error;
mod lexer;
mod parser;
mod static_analyzer;
mod translator;

fn main() {
    let filename = env::args().last().expect("no input file");

    if let Ok(code) = fs::read_to_string(filename.clone()) {
        println!("{}", code);

        let mut t = Translator::new(code.clone());

        match t.translate() {
            Ok(_) => {
                for i in t.instructions {
                    println!("{i}");
                }
            }
            Err(errors) => {
                const RED: &str = "\x1b[31m\x1b[1m";
                const CANCEL: &str = "\x1b[0m";
                const GREEN: &str = "\x1b[32m\x1b[1m";

                for error in errors {
                    println!(
                        "{filename}:{}:{}: {RED}error:{CANCEL} {}",
                        error.pos.line, error.pos.column, error.kind
                    );

                    let line = error.pos.line;
                    let len = (line as f64).log10().floor() as usize + 1;

                    println!(
                        " {line} | {}",
                        get_line_from(code.as_str(), error.pos.line_begin)
                    );
                    println!(
                        "{}| {}{RED}^{CANCEL}",
                        " ".repeat(len + 2),
                        " ".repeat(error.pos.column - 1),
                    );

                    match error.kind {
                        StaticErrorKind::ReDeclaringVariable { name, defined_at } => {
                            println!(
                                "{GREEN}note:{CANCEL} function \"{name}\" is declared at {}:{}:{}",
                                filename, defined_at.line, defined_at.column
                            );
                            println!(
                                " {} | {}",
                                defined_at.line,
                                get_line_from(code.as_str(), defined_at.line_begin)
                            )
                        }
                        StaticErrorKind::InvalidArgumentsCount {
                            expected_args_count: _,
                            received_args_count: _,
                            function_id,
                        } => {
                            println!(
                                "{GREEN}note:{CANCEL} function \"{}\" is declared at: {filename}:{}:{}",
                                function_id.value,
                                function_id.pos.line,
                                function_id.pos.column
                            );
                            println!(
                                " {} | {}",
                                function_id.pos.line,
                                get_line_from(code.as_str(), function_id.pos.line_begin)
                            )
                        }
                        _ => {}
                    }
                    println!();
                }
            }
        }
    } else {
        println!("no such file \"{filename}\"");
    }
}

fn get_line_from(code: &str, line_start: usize) -> &str {
    let mut len = 0;

    while line_start + len < code.len()
        && code.to_string()[line_start + len..]
            .chars()
            .next()
            .expect("")
            != '\n'
    {
        len += 1;
    }

    code[line_start..line_start + len].as_ref()
}
