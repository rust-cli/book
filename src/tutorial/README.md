# Learning Rust by Writing a Command Line App in 15 Minutes

This short tutorial will guide you through writing
a CLI (command line interface) application
in [Rust].
It will take you roughly fifteen minutes;
but feel free to skip parts you don't need to know right now
or jump in at any point.
You’ll learn all the essentials about how to get going,
and where to find more information.

What kind of project do you want to write?
How about we start with something simple:
Let’s write a small `grep` clone.
That is a tool that we can give a string and a path
and it’ll print only the lines that contain the given string.
Let’s call it `grrs` (pronounced “grass”).

<aside class="note">

**Note:**
This book is written for [Rust 2018].
The code examples can also be used on Rust 2015,
but you might need to tweak them a bit;
add `extern crate foo;` invocations, for example.

At the time of this writing,
to use Rust 2018 you have to use a nightly compiler version.
Enable it by adding `edition = "2018"`
to the `[package]` section of your `Cargo.toml` file
(as well as write `cargo-features = ["edition"]` at the top of it).

</aside>

[Rust]: https://rust-lang.org/
[Rust 2018]: https://rust-lang-nursery.github.io/edition-guide/
