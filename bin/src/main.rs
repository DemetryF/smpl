mod errors;

use std::{fs, process::Command};

use clap::Parser;

use smplc_backend_x86::compile;
use smplc_lexer::Lexer;
use smplc_parse::{parse, ParseError, TokenStream};
use smplc_semcheck::sem_check;
use smplc_translate::translate;
use smplc_typecheck::typecheck;

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
    let tokens = Lexer::new(code);
    let token_stream = match TokenStream::new(tokens) {
        Ok(token_stream) => token_stream,
        Err(err) => {
            let err = ParseError::from(err);
            output_error(filename, code, err.span, err.kind);
            return Err(());
        }
    };

    let stmts = match parse(token_stream) {
        Ok(stmts) => stmts,
        Err(err) => {
            output_error(filename, code, err.span, err.kind);
            return Err(());
        }
    };

    let hir = match sem_check(stmts) {
        Ok(stmts) => stmts,
        Err(err) => {
            output_error(filename, code, err.span, err.kind);
            return Err(());
        }
    };

    let thir = match typecheck(hir) {
        Ok(thir) => thir,
        Err(errors) => {
            for error in errors {
                output_error(filename, code, error.span, error.kind);
            }

            return Err(());
        }
    };

    let ir_code = translate(thir);

    if show_ir {
        todo!()
    }

    compile(ir_code).map_err(|_| ())
}

pub fn assembly(assembly: String, output_filename: String) {
    if let Err(err) = fs::write("./temp.asm", assembly) {
        eprintln!("Error: {err}");
        return;
    }

    if let Err(err) = Command::new("nasm")
        .args(["-f", "elf64", "./temp.asm", "-o", "temp.o"])
        .output()
    {
        eprintln!("Error: {err}");
        return;
    }

    if let Err(err) = Command::new("gcc")
        .args(["-no-pie", "temp.o", &format!("-o{output_filename}")])
        .output()
    {
        eprintln!("Error: {err}");
        return;
    }

    if let Err(err) = Command::new("rm").arg("./temp.asm").arg("temp.o").output() {
        eprintln!("Error: {err}");
    }
}
