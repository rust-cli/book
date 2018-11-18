# Communicating with machines

The power of command-line tools really comes to shine
when you are able to combine them.
This is not a new idea:
In fact, this is a sentence from the [Unix philosophy]:

> Expect the output of every program to become the input to another, as yet unknown, program.

[Unix philosophy]: https://en.wikipedia.org/wiki/Unix_philosophy

If our programs fulfil this expectation,
our users will be happy.
To make sure this works well,
we should provide not just pretty output for humans,
but also a version tailored to what other programs need.
Let's see how we can do this.

## Who's reading this?

The first question to ask is:
Is our output for a human in front of a colorful terminal,
or for another program?
To answer this,
we can use a crate like [atty]:

[atty]: https://crates.io/crates/atty

```rust
use atty::Stream;

if atty::is(Stream::Stdout) {
    println!("I'm a terminal");
} else {
    println!("I'm not");
}
```

Depending on who will read our output,
we can then add extra information.
Humans tend to like colors,
for example,
so if I run `ls` in a random Rust project,
I'll get something like

```console
$ ls
CODE_OF_CONDUCT.md   LICENSE-APACHE       examples
CONTRIBUTING.md      LICENSE-MIT          proptest-regressions
Cargo.lock           README.md            src
Cargo.toml           convey_derive        target
```

where the last three words are in blue (to show they are folders).
If I were to pipe this to a file,
or a program like `cat`,
`ls` will adapt its output.
Instead of using columns that fit my terminal window
it will print every entry on its own line.
It will also not emit any colors.

```console
$ ls | cat
CODE_OF_CONDUCT.md
CONTRIBUTING.md
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
README.md
convey_derive
examples
proptest-regressions
src
target
```

## Easy output formats for machines

Historically,
the only type of output command-line tools produced were strings.
This is usually fine for people in front of terminals,
who are able to read text
and reason about its meaning.
Other programs usually don't have that ability, though:
The only way for them to understand the output of a tool
like `ls`
is if the author of the program included a parser
that happens to work for whatever `ls` outputs.

This often means
that output was limited to what is easy to parse.
Formats like TSV,
where each record is on its own line,
and each line contains tab-separated content,
are very popular.
These simple formats based on lines of text
allow tools like `grep`
to be used on the output of tools like `ls`.
`| grep Cargo` doesn't care if your lines are from `ls` or file,
it will just filter line by line.

The downside of this is that you can't use
an easy `grep` invocation to filter all the directories that `ls` gave you.

## JSON output for machines

<aside class="todo">

**TODO:**
Talk about how outputting lines of JSON documents is an approach some tools take.
(see [#95](https://github.com/rust-lang-nursery/cli-wg/issues/95))

</aside>

## How to deal with input piped into us

<aside class="todo">

**TODO:**
Talk about how work with stdin
(see [#95](https://github.com/rust-lang-nursery/cli-wg/issues/95))

</aside>
