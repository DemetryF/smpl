mod errors;

use std::{fs, process::Command};

use clap::Parser;

use smplc_backend_x86::compile;
use smplc_lexer::lex;
use smplc_parse::{parse, TokenStream};
use smplc_semcheck::sem_check;
use smplc_translate::translate;

use errors::output_error;

#[derive(Parser, Debug)]
struct Args {
    filename: String,

    #[arg(short, long, default_value_t = String::from("a.out"))]
    output: String,
}

fn main() -> Result<(), ()> {
    let Args { filename, output } = Args::parse();

    let program = fs::read_to_string(filename.as_str())
        .map_err(|_| eprintln!("failed to open \"{filename}\""))?;

    let tokens =
        lex(&program).map_err(|err| output_error(&filename, &program, err.pos, err.char))?;

    let token_stream = TokenStream::new(tokens);

    let stmts =
        parse(token_stream).map_err(|err| output_error(&filename, &program, err.pos, err.kind))?;

    let hir = sem_check(stmts).map_err(|err| {
        output_error(&filename, &program, err.pos, err.kind);
    })?;

    let intermediate_code = translate(hir);

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

    Ok(())
}
