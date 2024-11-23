use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author = "Dario Le Hy <dario.lehy@ik.me>", version = "1.0", about = "Frutta programming language CLI")]
struct Args {
    input: Option<String>,
    #[arg(short, long)]
    #[arg(help = "Show the AST")]
    ast: bool,
}

fn main() {
    let args = Args::parse();
    let input_file = args.input.or_else(|| std::env::args().nth(1));

    if let Some(input_file) = input_file {
        let input = std::fs::read_to_string(input_file).expect("Failed to read input file");

        // Assuming you have a parser and VM module
        let expr = parser::Parser::parse(&input);
        if args.ast {
            println!("{:#?}", expr);
        }

        let mut vm = vm::VM::new();
        if let Ok(expr) = expr {
            vm.exec_statement(expr);
        }
    } else {
        run_repl();
    }
}

fn run_repl() {
    let mut vm = vm::VM::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        if input.trim().is_empty() {
            continue;
        }

        let expr = parser::Parser::parse(&input);
        match expr {
            Ok(expr) => {
                vm.exec_statement(expr);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}