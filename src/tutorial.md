# Learning Rust by Writing a Command Line App in 15 Minutes

Rust is a statically compiled, fast language with great tooling and a rapidly growing ecosystem.
That makes it a great fit for writing command line applications:
They should be small, portable, and quick to run.
Command line applications are also a great way to get started with learning Rust;
or if you want to introduce Rust to your team!

This page will guide you through writing a CLI (command line interface) application in Rust in roughly fifteen minutes.
You’ll learn all the essentials about how to get going,
and where to find more information.

## Project setup

If you haven’t already,
[install Rust](https://www.rust-lang.org/install.html) on your computer (it should only take a few minutes).
After that, open a terminal and navigate to the directory you want to put your application code into.

What kind of project to you want to write?
How about we start with something simple:
Let’s write a small `grep` clone.
That is a tool that we can give a string and a path and it’ll tell us which lines contain the string.
Let’s call it `grrs` (pronounced “grass”).

If you’ve already seen the basic Rust tutorials,
you might be inclined to start with `cargo new --bin my-cool-app`.
To save us some time,
we’ll instead start with a CLI-specific template:
`cargo generate --git https://github.com/rust-clique/cargo-template-cli`.
When you run this, it’ll ask you for a project name.

If look at the newly created `grrs` directory,
you’ll find a typical setup for a Rust project:

- A `Cargo.toml` file that contains metadata for our project, incl. a list of dependencies/external libraries we use.
- A `src/main.rs` file that is the entry point for our (main) binary.
- A `tests/` directory that will contain integration tests for our tool.

If you can execute `cargo run` in the `grrs` directory and see it greet you, you’re all set up.

## Parsing command line arguments

A typical invocation of our CLI tool will look like this:
`grrs foobar test.txt`.
You can think of CLI arguments as a data type.
In our case, we have two fields,
`pattern` (the string to look for),
and `path` (the file to look in).
In Rust, it is very common to try and structure programs around the data they deal with.
This is a good start:

```rust
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```

*Aside:* `PathBuf` is like a `String` but for file system paths that works cross-platform.

Now, we still need to actually get the arguments the user passed into this form.
One option would be manually parse the list of strings we get from the operating system,
but a much nicer way is to use one of the many available libraries.
As you can see in the `src/main.rs` file,
our templates already contains some code using `clap`,
and in particular use it’s “derive” feature.
This is quite nice:
All we have to do is annotate a struct and it’ll generate the code that parses the arguments into the fields.
Let’s add our fields to the `Cli` struct in the template 
and also some documentation comments along the way.
It’ll look like this:

```rust
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Clap)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
```

Right below the `Cli` struct our template contains its `main` function.
When the program starts, it will call this function.
The first line is

```rust
let args = Cli::parse();
```

This will try to parse the arguments the user gave when executing the program into our `Cli` struct.
You might be wondering what happens if this fails.
Give it a try!

## First implementation of** `**grrs**`

Right, now that we have our input data,
we can start to write our actual tool.
Let’s start by opening the file:

```rust
let content = std::fs::read_to_string(&args.path)?;
```

*Aside:* If the File can’t be read, the `?` will propagate the error and stop the function.

Now, let’s iterate over the lines and print each one that contains our pattern:

```rust
for line in content.lines() {
    if line.contains(args.pattern) {
        println!("{}", line);
    }
}
```

Give it a try: `cargo run -- main src/main.rs` should work now!

*Aside:* This is not the most performant implementation, and will read the whole file into memory.
Feel free to optimize it!
(One idea might be to use a `[BufReader](https://doc.rust-lang.org/1.27.0/std/io/struct.BufReader.html)` instead of `read_to_string()`.)

## Nicer error reporting

*TODO:* Replace `?` with `.with_context(|_| format!("could not read file {}", args.path))`

## Logging

To make it easier to understand what is happening in our program,
we might want to add some log statements.
This is usually easy while writing your application.
But it will become super helpful when running this program again in half a year.

*Aside:* Experience has shown that even mildly useful CLI programs can end up being used for years to come.
Especially if they were meant as a temporary solution.

## Testing

*TODO:* Talk about using assert_cli’s features to quickly run cargo binaries with different inputs and assert their outputs.
*TODO:* Talk about generating temp dirs with demo files.

## Packaging and rendering documentation

If you feel confident that your program is ready to for other people to use,
it is time to package it!

*TODO:* Talk about packaging on CI
*TODO:* Talk about automatically generating Man pages in a build script

