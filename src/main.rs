mod exe;
mod fasm;
mod ir;
mod lex;
mod parse;
use colored::Colorize;
use exe::Runner;
use fasm::FasmGenerator;
use ir::Representation;
use lex::Lexer;
use parse::SyntaxParser;
use std::{fs::read_to_string, io::Write, process::exit};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() > 3 {
        eprintln!("{}", "Too many arguments".red());
        help();
        exit(1);
    } else if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            help();
        } else {
            run_file(&args[1]);
        }
    } else if args[1] == "-c" || args[1] == "--compile" {
        match compile_file(&args[2]) {
            Ok(_) => println!("{}", "Compilation successful".green()),
            Err(e) => eprintln!("{}: {}", "Compilation failed".red(), e),
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

    let mut syntax = SyntaxParser::new();

    match syntax.parse(lexer.tokens()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.red());
            exit(1);
        }
    }

    let mut rep = Representation::new();
    rep.parse(lexer.tokens());

    for ins in &rep.instructions {
        println!("{:?}", ins);
    }

    let mut runner = Runner::new(lexer.tokens().clone());
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

    loop {
        print!("{} ", ">>>".green());
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
        if input.trim().starts_with("print ") {
            let s = input.replace("print ", "").replace(" ", "");

            let mut i = 0;
            for c in s.chars() {
                if !c.is_numeric() {
                    break;
                }
                i = i * 10 + (c as usize - '0' as usize);
            }

            if i == 0 {
                i = 10;
            }

            runner.print_tape(i);
            println!();
            continue;
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

        let mut syntax = SyntaxParser::new();

        match syntax.parse(lexer.tokens()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e.red());
                continue;
            }
        }
        runner.add(&mut lexer.tokens().clone());
        runner.run();
        println!();
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

fn compile_file(file: &String) -> Result<(), String> {
    if !file.ends_with(".bf") {
        return Err("File must have .bf extension".to_string());
    }

    let content = std::fs::read_to_string(file)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut lexer = Lexer::new(content);
    lexer.parse().map_err(|e| format!("Lexer error: {}", e))?;

    let mut syntax = SyntaxParser::new();
    syntax.parse(lexer.tokens())
          .map_err(|e| format!("Parser error: {}", e))?;

    let mut rep = Representation::new();
    rep.parse(lexer.tokens());

    let mut gen = FasmGenerator::new();
    let asm = gen.generate(&rep.instructions);

    let output_asm = file.replace(".bf", ".asm");
    std::fs::write(&output_asm, asm)
        .map_err(|e| format!("Failed to write ASM file: {}", e))?;

    // Output executable name will be the same as the input file but without extension
    let output_exe = file.replace(".bf", "");

    // Run FASM to compile directly to an executable
    let status = std::process::Command::new("fasm")
        .args([&output_asm, &output_exe])
        .status()
        .map_err(|e| format!("Failed to run FASM: {}", e))?;

    if !status.success() {
        return Err("FASM compilation failed".to_string());
    }

    // Make the output file executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&output_exe)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(&output_exe, perms)
            .map_err(|e| format!("Failed to set file permissions: {}", e))?;
    }

    // Delete the generated .asm file now that compilation was successful.
    std::fs::remove_file(&output_asm)
        .map_err(|e| format!("Failed to remove ASM file: {}", e))?;

    Ok(())
}