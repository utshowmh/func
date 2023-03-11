use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use func::{
    common::error::Error,
    frontend::{lexer::Lexer, parser::Parser},
    runtime::interpreter::Interpreter,
};

fn main() {
    run().unwrap_or_else(|err| err.report());
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => {
            run_repl();
        }

        2 => {
            let source_path = &args[1];
            if source_path == "-h" {
                print_usage(None);
            } else {
                run_file(source_path)?;
            }
        }
        _ => print_usage(Some("Invalid number of argument")),
    }

    Ok(())
}

fn repl_loop(interpreter: &mut Interpreter) -> Result<(), Error> {
    let mut line = String::new();
    loop {
        print!(":> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();

        let mut lexer = Lexer::new("stdin".to_string(), line.trim());
        let tokens = lexer.lex()?;

        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        interpreter.interpret(program)?;


        line.clear();
    }
}

fn run_repl() {
    println!("Press [Ctrl] + [c] exit.");
    let mut interpreter = Interpreter::new();
    loop {
        if let Err(err) = repl_loop(&mut interpreter) {
            err.report();
        }
    }
}

fn run_file(source_path: &str) -> Result<(), Error> {
    let source = read_to_string(source_path).unwrap();

    let mut lexer = Lexer::new(source_path.to_string(), &source);
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(program)?;

    Ok(())
}

fn print_usage(err: Option<&str>) {
    let usage = "
Usage:

func [source_file] | [options]

options:
-h :   prints help.
";
    if let Some(err) = err {
        eprintln!("Error: {}.", err);
        eprintln!("{}", usage);
    } else {
        println!("{}", usage);
    }
}
