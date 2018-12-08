# Project setup

If you haven’t already,
[install Rust] on your computer
(it should only take a few minutes).
After that, open a terminal and navigate to the directory
you want to put your application code into.

[install Rust]: https://www.rust-lang.org/tools/install

Start by running
`cargo new grrs`
in the directory you store your programming projects in.
If you look at the newly created `grrs` directory,
you’ll find a typical setup for a Rust project:

- A `Cargo.toml` file that contains metadata for our project,
  incl. a list of dependencies/external libraries we use.
- A `src/main.rs` file that is the entry point for our (main) binary.

If you can execute `cargo run` in the `grrs` directory
and get a "Hello World", you’re all set up.

## What it might look like

```console
$ cargo new grrs
     Created binary (application) `grrs` package
$ cd grrs/
$ cargo run
   Compiling grrs v0.1.0 (/Users/pascal/code/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/grrs`
Hello, world!
```
