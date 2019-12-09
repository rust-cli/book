# Parsing command line arguments

A typical invocation of our CLI tool will look like this:

```console
$ grrs foobar test.txt
```

We expect our program to look at `test.txt`
and print out the lines that contain `foobar`.
But how do we get these two values?

The text after the name of the program is often called
the "command line arguments",
or "command line flags"
(especially when they look like `--this`).
Internally, the operating system usually represents them
as a list of strings –
roughly speaking, they get separated by spaces.

There are many ways to think about these arguments,
and how to parse them
into something more easy to work with.
You will also need to tell the users of your program
which arguments they need to give
and in which format they are expected.

## Getting the arguments

The standard library contains the function
[`std::env::args()`] that gives you an [iterator] of the given arguments.
The first entry (at index `0`) will be the name your program was called as (e.g. `grrs`),
the ones that follow are what the user wrote afterwards.

[`std::env::args()`]: https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
[iterator]: https://doc.rust-lang.org/1.39.0/std/iter/index.html

Getting the raw arguments this way is quite easy (in file src/main.rs, after fn main() {):

```rust,ignore
{{#include cli-args-struct.rs:10:11}}
```

## CLI arguments as data type

Instead of thinking about them as a bunch of text,
it often pays off to think of CLI arguments as a custom data type
that represents the inputs to your program.

Look at `grrs foobar test.txt`:
There are two arguments,
first the `pattern` (the string to look for),
and then the `path` (the file to look in).

What more can we say about them?
Well, for a start, both are required.
We haven't talked about any default values,
so we expect our users to always provide two values.
Furthermore, we can say a bit about their types:
The pattern is expected to be a string,
while the second argument is expect to be path to a file.

In Rust, it is very common to structure programs around the data they deal with
so this way of looking at CLI arguments fits very well.
Let's start with this (in file src/main.rs, before fn main() {):

```rust,ignore
{{#include cli-args-struct.rs:3:7}}
```

This defines a new structure (a [`struct`])
that has two fields to store data in: `pattern`, and `path`.

[`struct`]: https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html

<aside>

**Aside:**
[`PathBuf`] is like a [`String`] but for file system paths that works cross-platform.

[`PathBuf`]: https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.39.0/std/string/struct.String.html

</aside>

Now, we still need to get the actual arguments our program got into this form.
One option would be to manually parse the list of strings we get from the operating system
and build the structure ourselves.
It would look something like this:

```rust,ignore
{{#include cli-args-struct.rs:10:15}}
```

This works, but it's not very convenient.
How would you deal with the requirement to support
`--pattern="foo"` or `--pattern "foo"`?
How would you implement `--help`?

## Parsing CLI arguments with StructOpt

A much nicer way is to use one of the many available libraries.
The most popular library for parsing command line arguments
is called [`clap`].
It has all the functionality you'd expect,
including support for sub-commands, shell completions, and great help messages.

The [`structopt`] library builds on `clap`
and provides a "derive" macro
to generate `clap` code for `struct` definitions.
This is quite nice:
All we have to do is annotate a struct
and it’ll generate the code that parses the arguments into the fields.

[`clap`]: https://clap.rs/
[`structopt`]: https://docs.rs/structopt

Let's first import `structopt` by adding
`structopt = "0.2.10"` to the `[dependencies]` section
of our `Cargo.toml` file.

Now, we can write `use structopt::StructOpt;` in our code,
and add `#[derive(StructOpt)]` right above our `struct Cli`.
Let's also write some documentation comments along the way.

It’ll look like this (in file src/main.rs, before fn main() {):

```rust,ignore
{{#include cli-args-structopt.rs:3:14}}
```

<aside class="node">

**Note:**
There are a lot of custom attributes you can add to fields.
For example,
we added one to tell structopt how to parse the `PathBuf` type.
To say you want to use this field for the argument after `-o` or `--output`,
you'd add `#[structopt(short = "o", long = "output")]`.
For more information,
see the [structopt documentation][`structopt`].

</aside>

Right below the `Cli` struct our template contains its `main` function.
When the program starts, it will call this function.
The first line is:

```rust,ignore
{{#include cli-args-structopt.rs:15:18}}
```

This will try to parse the arguments into our `Cli` struct.

But what if that fails?
That's the beauty of this approach:
Clap knows which fields to expect,
and what their expected format is.
It can automatically generate a nice `--help` message,
as well as give some great errors
to suggest you pass `--output` when you wrote `--putput`.

<aside class="note">

**Note:**
The `from_args` method is meant to be used in your `main` function.
When it fails,
it will print out an error or help message
and immediately exit the program.
Don't use it in other places!

</aside>

## This is what it may look like

Running it without any arguments:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 10.16s
     Running `target/debug/grrs`
error: The following required arguments were not provided:
    <pattern>
    <path>

USAGE:
    grrs <pattern> <path>

For more information try --help
```

We can pass arguments when using `cargo run` directly by writing them after  `--`:

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
```

As you can see,
there is no output.
Which is good:
That just means there is no error and our program ended.

<aside class="exercise">

**Exercise for the reader:**
Make this program output its arguments!

</aside>
