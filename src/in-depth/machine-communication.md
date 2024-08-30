# 與機器互動

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

**筆記:**
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
we can use a crate like [is-terminal]:

[is-terminal]: https://crates.io/crates/is-terminal

```rust,ignore
use is_terminal::IsTerminal as _;

if std::io::stdout().is_terminal() {
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
{{#include machine-communication.rs}}
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

**筆記:**
This is how Visual Studio Code uses _ripgrep_ for its code search.

</aside>

## How to deal with input piped into us

Let's say we have a program that reads the number of words in a file:

``` rust,ignore
{{#include machine-communication-wc.rs}}
```

It takes the path to a file, reads it line by line, and counts the number of
words separated by a space.

When you run it, it outputs the total words in the file:

``` console
$ cargo run README.md
Words in README.md: 47
```

But what if we wanted to count the number of words piped into the program?
Rust programs can read data passed in via stdin with the [Stdin
struct](https://doc.rust-lang.org/std/io/struct.Stdin.html) which you can
obtain via [the stdin function](https://doc.rust-lang.org/std/io/fn.stdin.html)
from the standard library. Similar to reading the lines of a file, it can read
the lines from stdin.

Here's a program that counts the words of what's piped in via stdin

``` rust,ignore
{{#include machine-communication-stdin.rs}}
```

If you run that program with text piped in, with `-` representing the intent to
read from `stdin`, it'll output the word count:

``` console
$ echo "hi there friend" | cargo run -- -
Words from stdin: 3
```

It requires that stdin is not interactive because we're expecting input that's
piped through to the program, not text that's typed in at runtime. If stdin is
a tty, it outputs the help docs so that it's clear why it doesn't work.
