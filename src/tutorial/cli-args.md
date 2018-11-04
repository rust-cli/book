# Parsing command line arguments

A typical invocation of our CLI tool will look like this:
`grrs foobar test.txt`.
We expect our program to look at `test.txt`
and print out the lines that contain `foobar`.
But how do we get these two values?

The text after the name of the program is often called
the "command line arguments",
or "command line flags"
(especially when they look like `--this`).
Internally, the operating system usually represents them
as a list of strings --
roughly speaking, the get separated by spaces.
There are many ways to think about these arguments,
and how to parse them
into something more easy to work with.
You will also need to tell the users of your program
which arguments they need to give
and in which format they are expected.

## CLI Arguments as data type

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
Further more, we can say a bit about their types:
The pattern is expected to be a string,
while the second argument is expect to be path to a file.

In Rust, it is very common to structure programs around the data they deal with
so this way of looking at CLI arguments fits very well.
Let's start with this:

```rust
#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```

This defines a new structure (a [`struct`])
that has two fields to store data in: `pattern`, and `path`.

[`struct`]: https://doc.rust-lang.org/1.27.2/book/second-edition/ch05-00-structs.html

<aside>

**Aside:**
[`PathBuf`] is like a [`String`] but for file system paths that works cross-platform.

[`PathBuf`]: https://doc.rust-lang.org/1.27.2/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.27.2/std/string/struct.String.html

</aside>

Now, we still need to get the actual arguments our program got into this form.
One option would be manually parse the list of strings we get from the operating system
and build the structure ourselves.
It would looks something like this:

```rust
let pattern = std::env::args().nth(1).expect("no pattern given");
let path = std::env::args().nth(2).expect("no path given");
let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
};
```

This works, but it's not very convenient.
How would you deal with the requirement to support
`--pattern="foo"` or `--pattern "foo"`?
How would you implement `--help`?

## Parsing CLI Arguments with Clap

A much nicer way is to use one of the many available libraries.
The most popular library for parsing command line arguments
is called [`clap`].
It has all the functionality you'd expect,
including support for sub-commands, shell completions, and great help messages.
The [`structopt`] library builds on `clap`
and provides a "derive" macro
to generate `clap` applications.

[`clap`]: https://clap.rs/
[`structopt`]: https://docs.rs/structopt

This is quite nice:
All we have to do is annotate a struct
and it’ll generate the code that parses the arguments into the fields.
Let’s add our fields to the `Cli` struct in the template 
and also write some documentation comments along the way.
It’ll look like this:

```rust
#[macro_use] extern crate structopt;
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
```

<aside class="node">

**Note:**
There are a lot of custom attributes you can add to fields.
For example,
we added one to tell structopt how to parse the `PathBuf` type.
To say you want to use this field for the argument after `-o` or `--output`,
you'd add `#[structopt(short = "o", long = "output")]`.
For more information
see the [structopt documentation][`structopt`].

</aside>

<aside class="todo">

**TODO:**
Ensure this works with clap3.
[Issue #64](https://github.com/rust-lang-nursery/cli-wg/issues/64)

</aside>

Right below the `Cli` struct our template contains its `main` function.
When the program starts, it will call this function.
The first line is:

```rust
let args = Cli::from_args();
```

This will try to parse the arguments into our `Cli` struct.

But what if that fails?
That's the beauty of this approach:
Clap knows which fields to expect,
and what their expected format is.
It can automatically generate a nice `--help` message,
as well a give great errors
to suggest you pass `--output` when you wrote `--putput`.

<aside class="note">

**Note:**
The `from_args` method is meant to be used in your `main` function.
When it fails,
it will print out an error or help message and exit the program.
Don't use it in other places!

</aside>

## This is what it may look like

![](./tutorial/cli-args.svg)

<aside class="todo">

**TODO:**
Use clap 3!
[Issue #64](https://github.com/rust-lang-nursery/cli-wg/issues/64)

</aside>
