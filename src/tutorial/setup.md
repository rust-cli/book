# Project setup

If you haven’t already,
[install Rust](https://www.rust-lang.org/install.html) on your computer
(it should only take a few minutes).
After that, open a terminal and navigate to the directory
you want to put your application code into.

If you’ve already seen the basic Rust tutorials,
you might be inclined to start with `cargo new --bin my-cool-app`.
To save us some time,
we’ll instead start with a CLI-specific template:
`cargo generate --git https://github.com/rust-clique/cargo-template-cli`.
When you run this, it’ll ask you for a project name
(please enter "grrs").

If look at the newly created `grrs` directory,
you’ll find a typical setup for a Rust project:

- A `Cargo.toml` file that contains metadata for our project,
  incl. a list of dependencies/external libraries we use.
- A `src/main.rs` file that is the entry point for our (main) binary.
- A `tests/` directory that will contain integration tests for our tool.

If you can execute `cargo run` in the `grrs` directory
and see it greet you, you’re all set up.

## What it might look like

<aside class="todo">

**Aside:** Update to use cargo-generate

</aside>

![](./tutorial/setup.svg)
