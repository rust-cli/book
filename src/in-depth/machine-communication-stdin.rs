use clap::{CommandFactory, Parser};
use std::{io::stdin, path::PathBuf};
use atty::Stream;

/// Count the number of lines in a file or stdin
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file to read, use - to read from stdin (must not be a tty)
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut word_count = 0;
    let mut file = args.file;

    if file == PathBuf::from("-") {
        if atty::is(Stream::Stdin) {
            Cli::command().print_help().unwrap();
            ::std::process::exit(0);
        } else {
            file = PathBuf::from("stdin");
            for line in stdin().lines() {
                let line = line.unwrap();
                if !line.trim().is_empty() {
                    word_count += line.split(' ').count();
                }
            }
        }
    } else {
        let content = std::fs::read_to_string(&file).unwrap();
        for line in content.lines() {
            word_count += line.split(' ').count();
        }
    }

    println!("Words from {}: {}", file.to_str().unwrap(), word_count)
}
