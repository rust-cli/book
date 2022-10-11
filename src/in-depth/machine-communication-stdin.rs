use clap::Parser;
use std::{io::stdin, path::PathBuf};

/// Count the number of lines in a file
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file to read, use - to read from stdin
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut word_count = 0;
    let mut file = args.file;

    if file == PathBuf::from("-") {
        file = PathBuf::from("stdin");
        for line in stdin().lines() {
            let line = line.unwrap();
            if !line.trim().is_empty() {
                word_count += line.split(' ').count();
            }
        }
    } else {
        let content = std::fs::read_to_string(&file).unwrap();
        for line in content.lines() {
            word_count += line.split(' ').count();
        }
    }

    println!("Total words from {}: {}", file.to_str().unwrap(), word_count)
}