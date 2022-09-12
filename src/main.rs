mod scanner;
mod token;
mod token_type;

use crate::scanner::Scanner;
use crate::token::Token;

use clap::Parser;
use std::io::Write;
use std::{fs, io};

#[derive(Parser)]
#[clap(author = "Prakhar Nagpal", version, about = "A very simple compiler")]
struct Args {
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

fn run_compiler(file: String) {
    println!("Smpl: compiling {}", file);
    let contents = fs::read_to_string(&file)
        .expect("Ensure that the file is present, and has the correct file format.");
    run(&contents)
}

fn run_interpreter() {
    println!("Welcome to Smpl(s) interpreter. A simple programming language");
    loop {
        print!("|> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => run(input.trim()),
            Err(error) => println!("Error!: {error}"),
        }
    }
}

fn run(input: &str) {
    let mut scanner = Scanner {
        source: input.trim_end().to_string(),
        tokens: vec![],
        line: 1,
        start_pos: 0,
        current_pos: 0,
        pos_x: 0,
    };
    let tokens: &Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}

fn main() {
    let args = Args::parse();
    match args.file {
        None => run_interpreter(),
        Some(f) => run_compiler(f),
    }
}
