mod backend;
mod frontend;

use std::fs::File;
use std::io::Write;
use std::{env, fs, io::stdout, process::exit};

use crate::backend::transpiler::Transpiler;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

// fn repl() {
//     print!("Welcome to bline's repl, type '\\leave' to exit\n");
//     loop {
//         print!("=> ");
//         stdout().flush().expect("Failed to flush std output");
//         let mut input = String::new();
//         stdin().read_line(&mut input).expect("error");
//         if input.trim() == "\\leave" {
//             exit(0)
//         }
//     }
// }

pub fn error(line: u32, column: u32, message: String) {
    report(line, column, message);
}

fn report(line: u32, column: u32, message: String) {
    eprintln!("\n| Error at: Ln {}, Col {}, {}", line, column, message);
}

fn make_c_file(code: String) {
    let mut file = File::create("prototype01.c").expect("well we fucked up");

    let _ = file.write_all(&code.into_bytes());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     repl();
    // }

    if args[1] == "--help" || args[1] == "-h" {
        println!(
            "This is the interpreter for bline, a programming language developed By: Vinícios"
        );
        stdout().flush().expect("Failed to flush");
        exit(0)
    }

    let filename: &str = &args[1];

    // match Path::new(filename).extension() {
    //     Some(ext) => {
    //         let ext = ext.to_str().unwrap_or("").to_lowercase();
    //         if ext != "bline" {
    //             eprintln!("Err: Wrong File type");
    //             exit(1);
    //         }
    //     }
    //     None => {
    //         eprintln!("Err: Wrong File type");
    //         exit(1);
    //     }
    // }

    match fs::read_to_string(filename) {
        Ok(result) => {
            let mut lexer_instance = Lexer::new(&result);
            lexer_instance.scan_source_code();

            let mut parser_instance = Parser::new(lexer_instance.token_list);
            parser_instance.parse_tokens();

            let mut transpiler_instance = Transpiler::new(parser_instance.abstract_syntax_tree);
            transpiler_instance.transpile_abstract_syntax_tree();

            make_c_file(transpiler_instance.c_src_code);
        }
        Err(err) => {
            eprintln!("{err}");
            exit(127)
        }
    }
}
