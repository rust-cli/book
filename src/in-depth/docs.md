# Rendering documentation for your CLI apps

Documentation for CLIs usually consists of
a `--help` section in the command
and a manual (`man`) page.

Both can be automatically generated
when using `clap` v3 (in unreleased alpha,
at time of writing), via the `man` backend.

```rust,ignore
#[derive(Clap)]
pub struct Head {
    /// file to load
    #[clap(parse(from_os_str))]
    pub file: PathBuf,
    /// how many lines to print
    #[clap(short = "n", default_value = "5")]
    pub count: usize,
}
```

Secondly, you need to use a `build.rs`
to generate the manual file at compile time
from the definition of your app
in code.

There are a few things to keep in mind
(such as how you want to package your binary)
but for now
we simply put the `man` file
next to our `src` folder.

```rust,ignore
use clap::IntoApp;
use clap_generate::gen_manuals;

#[path="src/cli.rs"]
mod cli;

fn main() {
    let app = cli::Head::into_app();
    for man in gen_manuals(&app) {
        let name = "head.1";
        let mut out = fs::File::create("head.1").unwrap();
        use std::io::Write;
        out.write_all(man.render().as_bytes()).unwrap();
    }
}
```

When you now compile your application
there will be a `head.1` file
in your project directory.

If you open that in `man`
you'll be able to admire your free documentation.
