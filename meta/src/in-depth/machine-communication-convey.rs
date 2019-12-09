// #[macro_use] extern crate convey;

use serde_derive::Serialize;
use structopt::StructOpt;
use convey::{json, human, render_for_humans, render_json, span};
use convey::components::{newline, text};

/// Demo various output things
#[derive(StructOpt)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[structopt(long = "json")]
    json: bool,
}

fn main() -> Result<(), failure::Error> {
    let args = Cli::from_args();
    let mut out = if args.json {
        convey::new().add_target(json::stdout()?)
    } else {
        convey::new().add_target(human::stdout()?)
    };

    #[derive(Serialize)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    impl convey::Render for ErrorMessage {
        render_for_humans!(self -> [
            span!(fg = "white", bg = "black", [text(self.code.to_string()), text(" "),]),
            span!(fg = "red", bg = "black", [text(&self.name),]),
            newline(),
            text("> "),
            text(&self.message),
        ]);

        render_json!();
    }

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("error"),
        message: String::from("Oh god no"),
    })?;

    Ok(())
}
