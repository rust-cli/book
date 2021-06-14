use clap::{AppSettings, Clap};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process;

/// A sample-program
///
/// The program is a show-case how to read data either from file or stdin and write data either to
/// file or stdout
#[derive(Clap)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Optional output file; default stdout
    #[clap(short, long)]
    output: Option<String>,
    /// Optional input file; default stdin
    input: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    let exit = match (opts.input, opts.output) {
        (None, None) => run(io::stdin(), io::stdout()),
        (None, Some(f)) => run(io::stdin(), open_output(f)),
        (Some(f), None) => run(open_input(f), io::stdout()),
        (Some(fin), Some(fout)) => run(open_input(fin), open_output(fout)),
    };

    if let Err(err) = exit {
        println!("{}", err);
        process::exit(1);
    }
}

fn open_input(f: String) -> File {
    File::open(f).expect("Could not open input file")
}
fn open_output(f: String) -> File {
    File::create(f).expect("Could not open output file")
}

fn run(input: impl Read, output: impl Write) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(input);
    let mut wtr = csv::Writer::from_writer(output);

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}
