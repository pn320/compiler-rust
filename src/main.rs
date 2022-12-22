#![feature(exclusive_range_pattern)]

mod ast;
mod parser;
mod tokenizer;
mod token;
mod token_type;

use crate::tokenizer::Scanner;
use crate::token::Token;

use crate::token_type::TokenType;
use clap::Parser;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[clap(author = "Prakhar Nagpal", version, about = "A very simple compiler")]
struct Args {
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

fn run_compiler(file: String) -> Result<()> {
    println!("Smpl: compiling {}", file);
    let contents = fs::read_to_string(&file)
        .expect("Ensure that the file is present, and has the correct file format.");
    run(&contents)
}

fn run_interpreter() -> Result<()> {
    println!(
        "{} {} {}",
        format!("Welcome to"),
        format!("Monk(s)").green(),
        format!("interpreter. A simple programming language")
    );
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("interpreter_hist.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("|> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                run(line.trim()).unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", format!("Session interrupted [SIGINT].").red());
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting Monk!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("interpreter_hist.txt")
}

fn run(input: &str) -> Result<()> {
    let mut scanner = Scanner::new(input.to_string(), vec![], 1, 0, 0, 0, init_keywords_hash_map());
    let tokens: &Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn init_keywords_hash_map() -> HashMap<String, TokenType> {
    let keywords: HashMap<String, TokenType> = HashMap::from([
        ("and".to_string(), TokenType::And),
        ("struct".to_string(), TokenType::Struct),
        ("else".to_string(), TokenType::Else),
        ("false".to_string(), TokenType::False),
        ("for".to_string(), TokenType::For),
        ("fn".to_string(), TokenType::Function),
        ("if".to_string(), TokenType::If),
        ("none".to_string(), TokenType::None),
        ("or".to_string(), TokenType::Or),
        ("print".to_string(), TokenType::Print),
        ("return".to_string(), TokenType::Return),
        ("this".to_string(), TokenType::This),
        ("true".to_string(), TokenType::True),
        ("let".to_string(), TokenType::Let),
        ("while".to_string(), TokenType::While),
    ]);
    return keywords;
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.file {
        None => run_interpreter(),
        Some(f) => run_compiler(f),
    }
}
