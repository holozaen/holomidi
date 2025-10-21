mod cli;
mod commands;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(|s| s.as_str()).unwrap_or("program");

    if args.len() < 2 {
        cli::print_usage(program);
        return;
    }

    match args[1].as_str() {
        "list" => {
            if let Err(err) = commands::list::execute() {
                eprintln!("Error: {}", err);
            }
        }
        "sysex" => {
            if let Err(err) = commands::sysex::execute(&args[2..]) {
                eprintln!("Error: {}", err);
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            cli::print_usage(program);
        }
    }
}