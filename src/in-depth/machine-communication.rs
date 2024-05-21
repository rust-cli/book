use clap::Parser;
use serde_json::json;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[arg(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::parse();
    if args.json {
        println!(
            "{}",
            json!({
                "type": "message",
                "content": "Hello world",
            })
        );
    } else {
        println!("Hello world");
    }
}
