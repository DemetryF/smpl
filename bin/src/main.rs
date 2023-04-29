use std::{fs, process::Command};

use backend::compile;
use frontend::parse;
use middleend::translate;

use clap::Parser;

/// Simple program to greet a person
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

    if let Ok(program) = fs::read_to_string(filename.as_str()) {
        let stmts = match parse(&program) {
            Ok(stmts) => stmts,
            Err(errors) => {
                for frontend::Error { kind, .. } in errors {
                    println!("{}", kind);
                }

                return;
            }
        };

        let intermediate_code = match translate(stmts) {
            Ok(code) => code,
            Err(errors) => {
                for middleend::Error { kind, .. } in errors {
                    println!("{}", kind);
                }
                return;
            }
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
    } else {
        println!("no such file \"{filename}\"");
    }
}
