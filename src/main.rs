mod exe;
mod ir;
mod lex;
mod parse;
use colored::Colorize;
use exe::Runner;
use lex::Lexer;
use parse::SyntaxParser;
use std::{fs::read_to_string, io::Write, process::exit};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() > 2 {
        eprintln!("{}", "Too many arguments".red());
        help();
        exit(1);
    } else if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            help();
        } else {
            run_file(&args[1]);
        }
    }
}

fn run_file(file: &String) {
    if !file.ends_with(".bf") {
        eprintln!("{}", "The extension of the file should be .bf".red());
        exit(1);
    }
    let c = read_to_string(file);
    let content: String;
    match c {
        Ok(con) => {
            content = con;
        }
        _ => {
            eprintln!("{}", "Unable to open the file".red());
            exit(1)
        }
    }

    let mut lexer = Lexer::new(content);

    match lexer.parse() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.red());
            exit(1);
        }
    }

    let mut syntax = SyntaxParser::new(lexer.tokens());

    match syntax.parse() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.red());
            exit(1);
        }
    }

    let mut runner = Runner::new(lexer.tokens());
    runner.run();
}

fn run_prompt() {
    println!("{}", "brainfuck interpreter".yellow());
    println!("Type {} to quit", "exit".red());
    match std::io::stdout().flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{}", "Something went wrong".red());
        }
    }
    let mut runner = Runner::new(vec![]);

    print!("{} ", ">>>".green());
    loop {
        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("{}", "Something went wrong".red());
            }
        }

        let mut input = "".into();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {}
            _ => {
                eprintln!("{}", "Unable to read the input".red());
                exit(1);
            }
        }
        if input.trim() == "exit" {
            exit(0);
        }

        let mut lexer = Lexer::new(input);

        match lexer.parse() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e.red());
                continue;
            }
        }

        let mut syntax = SyntaxParser::new(lexer.tokens());

        match syntax.parse() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e.red());
                continue;
            }
        }
        runner.add(&mut lexer.tokens());
        runner.run();
        println!();
        print!("{} ", ">>>".green());
    }
}

fn help() {
    println!("          {}", "Brainfuck".blue().bold());
    println!(
        "Usage: {}",
        "brainfuck <subcommand>\n".green().bold().underline()
    );

    println!("          Subcommands");
    println!("<null>                Runs the brainfuck CLI");
    println!("{}             Runs the source code", "<file>.bf".yellow());
    println!("{}            Prints this message", "-h, --help".yellow());
}
