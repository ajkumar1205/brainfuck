mod exe;
mod lex;
mod parse;
use clap::Parser;
use exe::Runner;
use lex::Lexer;
use parse::SyntaxParser;
use std::{fs::read_to_string, process::exit};

/// It compiles the program and give the <filename>.exe file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file to compile
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = &args.file;

    if !file.ends_with(".bf") {
        eprintln!("The extension of the file should be .bf");
        exit(1);
    }

    let content = read_to_string(args.file)?;
    
    let mut lexer = Lexer::new(content);

    lexer.parse();

    let mut syntax = SyntaxParser::new(lexer.tokens());

    syntax.parse();

    let mut runner = Runner::new(lexer.tokens());
    runner.run();

    Ok(())
}
