# Communicating with machines

The power of command-line tools really comes to shine
when you are able to combine them.
This is not a new idea:
In fact, this is a sentence from the [Unix philosophy]:

> Expect the output of every program to become the input to another, as yet unknown, program.

[Unix philosophy]: https://en.wikipedia.org/wiki/Unix_philosophy

If our programs fulfill this expectation,
our users will be happy.
To make sure this works well,
we should provide not just pretty output for humans,
but also a version tailored to what other programs need.
Let's see how we can do this.

<aside>

**Aside:**
Make sure to read [the chapter on CLI output][output]
in the tutorial first.
It covers how to write output to the terminal.

[output]: ../tutorial/output.html

</aside>

## Who's reading this?

The first question to ask is:
Is our output for a human in front of a colorful terminal,
or for another program?
To answer this,
we can use a crate like [atty]:

[atty]: https://crates.io/crates/atty

```rust,ignore
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
if you run `ls` in a random Rust project,
you might see something like this:

```console
$ ls
CODE_OF_CONDUCT.md   LICENSE-APACHE       examples
CONTRIBUTING.md      LICENSE-MIT          proptest-regressions
Cargo.lock           README.md            src
Cargo.toml           convey_derive        target
```

Because this style is made for humans,
in most configurations
it'll even print some of the names (like `src`) in color
to show that they are directories.
If you instead pipe this to a file,
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
Formats like TSV (tab-separated values),
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
For that, each directory item would need to carry additional data.

## JSON output for machines

Tab-separated values is a simple way
to output structured data
but it requires the other program to know which fields to expect
(and in which order)
and it's difficult to output messages of different types.
For example,
let's say our program wanted to message the consumer
that it is currently waiting for a download,
and afterwards output a message describing the data it got.
Those are very different kinds of messages
and trying to unify them in a TSV output
would require us to invent a way to differentiate them.
Same when we wanted to print a message that contains two lists
of items of varying lengths.

Still,
it's a good idea to choose a format that is easily parsable
in most programming languages/environments.
Thus,
over the last years a lot of applications gained the ability
to output their data in [JSON].
It's simple enough that parsers exist in practically every language
yet powerful enough to be useful in a lot of cases.
While its a text format that can be read by humans,
a lot of people have also worked on implementations that are very fast at
parsing JSON data and serializing data to JSON.

[JSON]: https://www.json.org/

In the description above,
we've talked about "messages" being written by our program.
This is a good way of thinking about the output:
Your program doesn't necessarily only output one blob of data
but may in fact emit a lot of different information
while it is running.
One easy way to support this approach when outputting JSON
is to write one JSON document per message
and to put each JSON document on new line
(sometimes called [Line-delimited JSON][jsonlines]).
This can make implementations as simple as using a regular `println!`.

[jsonlines]: https://en.wikipedia.org/wiki/JSON_streaming#Line-delimited_JSON

Here's a simple example,
using the `json!` macro from [serde_json]
to quickly write valid JSON in your Rust source code:

[serde_json]: https://crates.io/crates/serde_json

```rust,ignore
{{#include machine-communication.rs:1:22}}
```

And here is the output:

```console
$ cargo run -q
Hello world
$ cargo run -q -- --json
{"content":"Hello world","type":"message"}
```

(Running `cargo` with `-q` suppresses its usual output.
The arguments after `--` are passed to our program.)

### Practical example: ripgrep

_[ripgrep]_ is a replacement for _grep_ or _ag_, written in Rust.
By default it will produce output like this:

[ripgrep]: https://github.com/BurntSushi/ripgrep

```console
$ rg default
src/lib.rs
37:    Output::default()

src/components/span.rs
6:    Span::default()
```

But given `--json` it will print:

```console
$ rg default --json
{"type":"begin","data":{"path":{"text":"src/lib.rs"}}}
{"type":"match","data":{"path":{"text":"src/lib.rs"},"lines":{"text":"    Output::default()\n"},"line_number":37,"absolute_offset":761,"submatches":[{"match":{"text":"default"},"start":12,"end":19}]}}
{"type":"end","data":{"path":{"text":"src/lib.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":137622,"human":"0.000138s"},"searches":1,"searches_with_match":1,"bytes_searched":6064,"bytes_printed":256,"matched_lines":1,"matches":1}}}
{"type":"begin","data":{"path":{"text":"src/components/span.rs"}}}
{"type":"match","data":{"path":{"text":"src/components/span.rs"},"lines":{"text":"    Span::default()\n"},"line_number":6,"absolute_offset":117,"submatches":[{"match":{"text":"default"},"start":10,"end":17}]}}
{"type":"end","data":{"path":{"text":"src/components/span.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":22025,"human":"0.000022s"},"searches":1,"searches_with_match":1,"bytes_searched":5221,"bytes_printed":277,"matched_lines":1,"matches":1}}}
{"data":{"elapsed_total":{"human":"0.006995s","nanos":6994920,"secs":0},"stats":{"bytes_printed":533,"bytes_searched":11285,"elapsed":{"human":"0.000160s","nanos":159647,"secs":0},"matched_lines":2,"matches":2,"searches":2,"searches_with_match":2}},"type":"summary"}
```

As you can see,
each JSON document is an object (map) containing a `type` field.
This would allow us to write a simple frontend for `rg`
that reads these documents as they come in and show the matches
(as well the files they are in)
even while _ripgrep_ is still searching.

<aside>

**Aside:**
This is how Visual Studio Code uses _ripgrep_ for its code search.

</aside>

## Abstract over human and machine output

[convey] is an in-development library
that tries to make it easier to output messages
in formats suitable for both humans and machines.
You define your own message types,
and implement a `Render` trait
(either manually, with the help of macros, or using a derive attribute)
to say how they should be formatted.
Currently,
it supports printing human output
(incl. auto-detecting whether it should be colored),
writing JSON documents
(either to the `stdout` or to a file),
or both at the same time.

[convey]: https://crates.io/crates/convey

Even if you do not use this library,
it might be a good idea to write a similar abstraction
that fits your use case.

## How to deal with input piped into us

<aside class="todo">

**TODO:**
Talk about how work with stdin
(see [#95](https://github.com/rust-lang-nursery/cli-wg/issues/95))

</aside>
