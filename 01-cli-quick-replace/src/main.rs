use std::env;
use text_colorizer::*;


fn main() {
    let args = parse_arguments();
    println!("{:?}", args);
}

fn print_cli_usage() {
    eprintln!("{} - change ocurrences of one string into another.", "cli-quick-replace".green());
    eprintln!("Usage: cli-quick-replace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_cli_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.", "Error: ".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments { 
        target: args[0].clone(), 
        replacement: args[1].clone(), 
        filename: args[2].clone(), 
        output: args[3].clone()
    }
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}