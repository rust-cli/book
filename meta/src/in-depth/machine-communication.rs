use structopt::StructOpt;
use serde_json::json;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::from_args();
    if args.json {
        println!("{}", json!({
            "type": "message",
            "content": "Hello world",
        }));
    } else {
        println!("Hello world");
    }
}
