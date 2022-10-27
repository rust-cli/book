use clap::{CommandFactory, Parser};
use std::{io::{stdin, BufReader, BufRead}, path::PathBuf, fs::File};
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

    let word_count;
    let mut file = args.file;

    if file == PathBuf::from("-") {
        if atty::is(Stream::Stdin) {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        file = PathBuf::from("<stdin>");
        word_count = words_in_buf_reader(BufReader::new(stdin().lock()));
    } else {
        word_count = words_in_buf_reader(BufReader::new(File::open(&file).unwrap()));
    }

    println!("Words from {}: {}", file.to_string_lossy(), word_count)
}

fn words_in_buf_reader<R: BufRead>(buf_reader: R) -> usize {
    let mut count = 0;
    for line in buf_reader.lines() {
        count += line.unwrap().split(' ').count()
    }
    count
}
