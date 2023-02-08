use std::{env::args, fs::read_to_string, process::exit};

use func::{
    common::error::Error,
    fronend::{lexer::Lexer, parser::Parser},
};

fn main() {
    run().unwrap_or_else(|err| err.report());
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => run_file(&args[1])?,
        _ => print_usage(Some("Invalid number of argument")),
    }

    Ok(())
}

fn run_file(source_path: &str) -> Result<(), Error> {
    let source = read_to_string(source_path).unwrap_or_else(|_| {
        print_usage(Some(&format!("Could not open file from `{}`", source_path)));
        exit(64);
    });

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    println!("{:#?}", program);

    Ok(())
}

fn print_usage(err: Option<&str>) {
    let usage = "
Usage:          func [file] [option]

file:           path to source file.
option:         [not available].
    ";
    if let Some(err) = err {
        eprintln!("Error: {}.", err);
        eprintln!("{}", usage);
    } else {
        println!("{}", usage);
    }
}
