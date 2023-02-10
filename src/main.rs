use std::{
    env::args,
    fs::{read_to_string, File},
    io::Write,
    process::exit,
};

use func::{
    common::error::Error,
    frontend::{lexer::Lexer, parser::Parser},
    generator::generator::Generator,
};

fn main() {
    run().unwrap_or_else(|err| err.report());
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        2 => {
            let source_path = &args[1];
            run_file(&source_path, None)?;
        }
        4 => {
            let flag = &args[2];
            if flag == "-o" {
                let source_path = &args[1];
                let output_path = &args[3];
                run_file(&source_path, Some(output_path))?;
            } else {
                print_usage(Some(&format!("Invalid flag {}", flag)));
            }
        }
        _ => print_usage(Some("Invalid number of argument")),
    }

    Ok(())
}

fn run_file(source_path: &str, output_path: Option<&str>) -> Result<(), Error> {
    let source = read_to_string(source_path).unwrap_or_else(|_| {
        print_usage(Some(&format!("Could not open file from `{}`", source_path)));
        exit(64);
    });

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.lex()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    let compiled_code = Generator::generate(program)?;

    if let Some(output_path) = output_path {
        let mut output_file = File::create(output_path).unwrap_or_else(|_| {
            print_usage(Some(&format!("Could not create file `{}`", output_path)));
            exit(64);
        });
        write!(output_file, "{}", compiled_code).unwrap_or_else(|_| {
            eprintln!("Could not write to file `{}`", output_path);
            exit(64);
        });
    } else {
        let mut output_file = File::create("output.c").unwrap_or_else(|_| {
            print_usage(Some(&format!("Could not open file from `{}`", source_path)));
            exit(64);
        });
        write!(output_file, "{}", compiled_code).unwrap_or_else(|_| {
            eprintln!("Could not write to file `output.c`");
            exit(64);
        });
    }

    Ok(())
}

fn print_usage(err: Option<&str>) {
    let usage = "
Usage:
func [source_file] [options]

options:
-o [output_file]   :   writes compiled code in file.
    ";
    if let Some(err) = err {
        eprintln!("Error: {}.", err);
        eprintln!("{}", usage);
    } else {
        println!("{}", usage);
    }
}
