use std::env;
use std::fs;
use text_colorizer::*;
use regex::Regex;

fn main() {
    let args = parse_arguments();
    
    let data = match fs::read_to_string(&args.filename) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error: ".red().bold(), args.filename, error);
            exit();
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{} failed to replace text: {:?}", "Error: ".red().bold(), error);
            exit();
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(error) => {
            eprintln!("{} failed to write to file '{}': {:?}", "Error: ".red().bold(), args.filename, error);
            exit();
        }
    }

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
        exit();
    }

    Arguments { 
        target: args[0].clone(), 
        replacement: args[1].clone(), 
        filename: args[2].clone(), 
        output: args[3].clone()
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn exit() -> ! {
    std::process::exit(1)
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}