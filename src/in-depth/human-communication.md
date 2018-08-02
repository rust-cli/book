# Communicating with humans

## When everything is fine

<aside class="todo">

**TODO:**
Style of writing:
- informative and concise
- easy to parse
- consistent: use same prefixes, sentence structure

</aside>

## When it's hard to tell what's going on

<aside class="todo">

**TODO:**
Log messages:
- levels
- consistent: use same prefixes, sentence structure
- provide enough context
- `--verbose`

</aside>

## When panicking

One aspect often forgotten is that
your program also outputs something when it crashes.
In Rust, "crashes" are most often "panics"
(i.e., "controlled crashing"
in contrast to "the operating system killed the process").
By default,
when a panic occurs,
a "panic handler" will print some information to the console.

For example,
if you create a new binary project
with `cargo new --bin foo`
and replace the content of `fn main` with `panic!("Hello World")`,
you get this when you run your program:

```console
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This is useful information to you, the developer.
(Surprise: the program crashed because of line 2 in your `main.rs` file).
But for a user who doesn't even have access to the source code,
this is not very valuable.
In fact, it most likely is just confusing.
That's why it's a good idea to add a custom panic handler,
that provides a bit more end-user focussed output.

One library that does just that is called [human-panic].
To add it to your CLI project,
you import it
and call the `setup_panic!()` macro
at the beginning of your `main` function:

```rust
use human_panic::setup_panic;

fn main() {
   setup_panic!();

   panic!("Hello world")
}
```

This will now show a very friendly message,
and tells the user what they can do:

```console
Well, this is embarrassing.

foo had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/n3/dkk459k908lcmkzwcmq0tcv00000gn/T/report-738e1bec-5585-47a4-8158-f1f7227f0168.toml". Submit an issue or email with the subject of "foo Crash Report" and include the report as an attachment.

- Authors: Your Name <your.name@example.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

[human-panic]: https://crates.io/crates/human-panic
