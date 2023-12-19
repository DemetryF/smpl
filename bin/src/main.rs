mod errors;

use std::{fs, process::Command};

use clap::Parser;

use backend::compile;
use middleend::translate;
use smplc_lexer::lex;
use smplc_parse::{parse, TokenStream};

use errors::output_error;

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

    let Ok(tokens) =
        lex(&program).map_err(|err| output_error(&filename, &program, err.pos, err.char))
    else {
        return;
    };

    let token_stream = TokenStream::new(tokens);

    let Ok(stmts) =
        parse(token_stream).map_err(|err| output_error(&filename, &program, err.pos, err.kind))
    else {
        return;
    };

    let Ok(intermediate_code) = translate(stmts).map_err(|errors| {
        for middleend::Error { kind, pos } in errors {
            output_error(&filename, &program, pos, kind)
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
