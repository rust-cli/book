# Communicating with humans

Make sure to read [the chapter on CLI output][output]
in the tutorial first.
It covers how to write output to the terminal,
while this chapter will talk about _what_ to output.

[output]: ../tutorial/output.html

## When everything is fine

It is useful to report on the application's progress
even when everything is fine.
Try to be informative and concise in these messages.
Don't use overly technical terms in the logs.
Remember:
the application is not crashing
so there's no reason for users to look up errors.

Most importantly,
be consistent in the style of communication.
Use the same prefixes and sentence structure
to make the logs easily skimmable.

Try to let your application output tell a story
about what it's doing
and how it impacts the user.
This can involve showing a timeline of steps involved
or even a progress bar and indicator for long-running actions.
The user should at no point
get the feeling that the application is doing something mysterious
that they cannot follow.

## When it's hard to tell what's going on

When communicating non-nominal state it's important to be consistent.
A heavily logging application that doesn't follow strict logging levels
provides the same amount, or even less information
than a non-logging application.

Because of this,
it's important to define the severity of events
and messages that are related to it;
then use consistent log levels for them.
This way users can select the amount of logging themselves
via `--verbose` flags
or environment variables (like `RUST_LOG`).

The commonly used `log` crate
[defines][log-levels] the following levels
(ordered by increasing severity):

- trace
- debug
- info
- warning
- error

It's a good idea to think of _info_ as the default log level.
Use it for, well, informative output.
(Some applications that lean towards a more quiet output style
might only show warnings and errors by default.)

Additionally,
it's always a good idea to use similar prefixes
and sentence structure across log messages,
making it easy to use a tool like `grep` to filter for them.
A message should provide enough context by itself
to be useful in a filtered log
while not being *too* verbose at the same time.

[log-levels]: https://docs.rs/log/0.4.4/log/enum.Level.html

### Example log statements

```console
error: could not find `Cargo.toml` in `/home/you/project/`
```

```console
=> Downloading repository index
=> Downloading packages...
```

The following log output is taken from [wasm-pack]:

```console
 [1/7] Adding WASM target...
 [2/7] Compiling to WASM...
 [3/7] Creating a pkg directory...
 [4/7] Writing a package.json...
 > [WARN]: Field `description` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `repository` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `license` is missing from Cargo.toml. It is not necessary, but recommended
 [5/7] Copying over your README...
 > [WARN]: origin crate has no README
 [6/7] Installing WASM-bindgen...
 > [INFO]: wasm-bindgen already installed
 [7/7] Running WASM-bindgen...
 Done in 1 second
```

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
that provides a bit more end-user focused output.

One library that does just that is called [human-panic].
To add it to your CLI project,
you import it
and call the `setup_panic!()` macro
at the beginning of your `main` function:

```rust,ignore
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
[wasm-pack]: https://crates.io/crates/wasm-pack
