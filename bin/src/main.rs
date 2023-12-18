mod errors;

use std::{fs, process::Command};

use clap::Parser;

use backend::compile;
use frontend::{parse, token_stream::TokenStream};
use middleend::translate;
use smplc_lexer::lex;

use errors::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value_t = String::from("a.out"))]
    output: String,
}

fn main() {
    let Args { filename, output } = Args::parse();

    let Ok(program) = fs::read_to_string(filename.as_str()) else {
        eprintln!("faield to open \"{filename}\"");
        return;
    };

    let Ok(tokens) = lex(&program).map_err(|err| {
        let error = Error {
            filename: &filename,
            code: &program,
            pos: err.pos,
            kind: err,
        };

        eprintln!("{error}");
    }) else {
        return;
    };

    let token_stream = TokenStream::new(tokens);

    let Ok(stmts) = parse(token_stream).map_err(|error| {
        let error = Error {
            filename: &filename,
            code: &program,
            pos: error.pos,
            kind: error.kind,
        };

        eprintln!("{error}");
    }) else {
        return;
    };

    let Ok(intermediate_code) = translate(stmts).map_err(|errors| {
        for middleend::Error { kind, pos } in errors {
            let error = Error {
                filename: &filename,
                code: &program,
                pos,
                kind,
            };

            eprintln!("{error}")
        }
    }) else {
        return;
    };

    let assembly = compile(intermediate_code).unwrap();

    fs::write("./temp.asm", assembly).unwrap();

    Command::new("nasm")
        .arg("-f elf64")
        .arg("./temp.asm")
        .arg("-o temp.o")
        .output()
        .unwrap();

    Command::new("gcc")
        .arg("-no-pie")
        .arg("temp.o")
        .arg(format!("-o{output}"))
        .output()
        .unwrap();

    Command::new("rm")
        .arg("./temp.asm")
        .arg("temp.o")
        .output()
        .unwrap();
}
