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
The code examples in this tutorial use features that are part of [Rust 2018].
Right now, this means you have to use a nightly compiler version
and add `edition = "2018"` to you `Cargo.toml`
(as well as write `cargo-features = ["edition"]` at the top of it).

It should be easy to write the same code in Rust 2015
-- the most significant difference is just that
I'm able to omit `extern crate foo;` lines.

</aside>

[Rust]: https://rust-lang.org/
[Rust 2018]: https://rust-lang-nursery.github.io/edition-guide/
