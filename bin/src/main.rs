mod errors;

use std::{fs, process::Command};

use clap::Parser;

use smplc_ast::Span;
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

    #[arg(long, default_value_t = false)]
    show_ir: bool,
}

fn main() {
    let Args {
        filename,
        output,
        show_ir,
    } = Args::parse();

    let program = match fs::read_to_string(filename.as_str()) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    let Ok(asm_code) = generate_asm(&program, &filename, show_ir) else {
        return;
    };

    assembly(asm_code, output);
}

pub fn generate_asm(code: &str, filename: &str, show_ir: bool) -> Result<String, ()> {
    let tokens = {
        match lex(&code) {
            Ok(tokens) => tokens,
            Err(err) => {
                output_error(&filename, &code, Span::with_len(err.pos, 1), err.char);
                return Err(());
            }
        }
    };

    let token_stream = TokenStream::new(tokens);

    let stmts = match parse(token_stream) {
        Ok(stmts) => stmts,
        Err(err) => {
            output_error(&filename, &code, err.span, err.kind);
            return Err(());
        }
    };

    let hir = match sem_check(stmts) {
        Ok(stmts) => stmts,
        Err(err) => {
            output_error(&filename, &code, err.span, err.kind);
            return Err(());
        }
    };

    let (ir_code, types) = translate(hir);

    if show_ir {
        println!("{ir_code}");
    }

    compile(ir_code, types).map_err(|_| ())
}

pub fn assembly(assembly: String, output_filename: String) {
    fs::write("./temp.asm", assembly).unwrap();

    dbg!(Command::new("nasm")
        .args(["-f", "elf64", "./temp.asm", "-o", "temp.o"])
        .output())
    .unwrap();

    dbg!(Command::new("gcc")
        .args(["-no-pie", "temp.o", &format!("-o{output_filename}")])
        .output())
    .unwrap();

    Command::new("rm")
        // .arg("./temp.asm")
        .arg("temp.o")
        .output()
        .unwrap();
}
