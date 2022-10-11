use clap::Parser;
use std::path::PathBuf;

/// Count the number of lines in a file
#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// The path to the file to read
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut word_count = 0;
    let file = args.file;

    for line in std::fs::read_to_string(&file).unwrap().lines() {
        word_count += line.split(' ').count();
    }

    println!("Words in {}: {}", file.to_str().unwrap(), word_count)
}
