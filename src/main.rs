mod lex;
mod exe;
use clap::Parser;
use lex::Lexer;
use exe::Runner;
use std::fs::read_to_string;

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
        panic!("The extension of the file should be .bf");
    }

    let content = read_to_string(args.file)?;
    println!("{}", content);
    let mut lexer = Lexer::new(content);

    lexer.parse();

    let mut runner = Runner::new(lexer.tokens());
    runner.run();

    Ok(())
}
