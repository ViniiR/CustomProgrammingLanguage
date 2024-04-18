mod backend;
mod frontend;

use std::io::Write;
use std::{env, fs, io::stdout, process::exit};

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
            dbg!(parser_instance.abstract_syntax_tree);
        }
        Err(err) => {
            eprintln!("{err}");
            exit(127)
        }
    }
}
