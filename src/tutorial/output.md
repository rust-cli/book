# 輸出

## 在終端機下輸出 "Hello World"

```rust
println!("Hello World");
```

嗯，這很容易。
太好了，進入下一個主題。

## 使用 `println!`

您幾乎可以在終端機下使用 `println!` 巨集輸出所有您想輸出的東西。

這個巨集有一些非常驚人的功能，而且還有特殊的語法。
它希望您編寫一個字串文字作為第一個參數，包含將要填入的佔位符號
再透過後面作為進一步參數的參數值。

範例:

```rust
let x = 42;
println!("My lucky number is {}.", x);
```

將會輸出

```console
My lucky number is 42.
```

上面字串中的大括弧（ `{}` ）就是佔位符號中的一種。 
這是預設的佔位符號類型， 它嘗試以人類可讀的方式來輸出給定的參數的值。 
對於數字和字串，這會很好用， 
但並不是所有的型別都可以。 
這就是為什麼還有一個 "除錯表示(debug representation)"， 
你可以使用這個佔位符號來呼叫它 `{:?}`。

範例

```rust
let xs = vec![1, 2, 3];
println!("The list is: {:?}", xs);
```

將會輸出

```console
The list is: [1, 2, 3]
```

如果你想在偵錯和日誌中輸出自己建置的類型，
大部分情況下你可以在類型定義上新增 `#[derive(Debug)]` 屬性。

<aside>

**筆記:**

"使用者友善(User-friendly)" 的輸出是使用 [`Display`] 特性完成的，
除錯輸出（適用於開發者）使用 [`Debug`] 特徵。
您可以在 `println!` 中找到更多關於可以使用的語法的資訊。
在[`std::fmt`模組的文檔][std::fmt]中。

[`Display`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Display.html
[`Debug`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Debug.html
[std::fmt]: https://doc.rust-lang.org/1.39.0/std/fmt/index.html

</aside>

## 輸出錯誤

輸出錯誤的部分應透過 stderr 完成， 
以便使用者和其它工具更方便的地將輸出通過管道傳輸到文件或更多的工具中。

<aside>

**Note:**
On most operating systems,
a program can write to two output streams, `stdout` and `stderr`.
`stdout` is for the program's actual output,
while `stderr` allows errors and other messages to be kept separate from `stdout`.
That way,
output can be stored to a file or piped to another program
while errors are shown to the user.

</aside>

In Rust this is achieved
with `println!` and `eprintln!`,
the former printing to `stdout`
and the latter to `stderr`.

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

<aside>

**Beware**: Printing [escape codes] can be dangerous,
putting the user's terminal into a weird state.
Always be careful when manually printing them!

[escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

Ideally you should be using a crate like `ansi_term`
when dealing with raw escape codes
to make your (and your user's) life easier.

</aside>

## A note on printing performance

Printing to the terminal is surprisingly slow!
If you call things like `println!` in a loop,
it can easily become a bottleneck in an otherwise fast program.
To speed this up,
there are two things you can do.

First,
you might want to reduce the number of writes
that actually "flush" to the terminal.
`println!` tells the system to flush to the terminal _every_ time,
because it is common to print each new line.
If you don't need that,
you can wrap your `stdout` handle in a [`BufWriter`]
which by default buffers up to 8 kB.
(You can still call `.flush()` on this `BufWriter`
when you want to print immediately.)

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
```

Second,
it helps to acquire a lock on `stdout` (or `stderr`)
and use `writeln!` to print to it directly.
This prevents the system from locking and unlocking `stdout` over and over again.

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = stdout.lock(); // acquire a lock on it
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
```

You can also combine the two approaches.

[`BufWriter`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufWriter.html

## Showing a progress bar

Some CLI applications run less than a second,
others take minutes or hours.
If you are writing one of the latter types of programs,
you might want to show the user that something is happening.
For this, you should try to print useful status updates,
ideally in a form that can be easily consumed.

Using the [indicatif] crate,
you can add progress bars
and little spinners to your program.
Here's a quick example:

```rust,ignore
{{#include output-progressbar.rs:1:9}}
```

See the [documentation][indicatif docs]
and [examples][indicatif examples]
for more information.

[indicatif]: https://crates.io/crates/indicatif
[indicatif docs]: https://docs.rs/indicatif
[indicatif examples]: https://github.com/console-rs/indicatif/tree/main/examples

## Logging

To make it easier to understand what is happening in our program,
we might want to add some log statements.
This is usually easy while writing your application.
But it will become super helpful when running this program again in half a year.
In some regard,
logging is the same as using `println!`,
except that you can specify the importance of a message.
The levels you can usually use are _error_, _warn_, _info_, _debug_, and _trace_
(_error_ has the highest priority, _trace_ the lowest).

To add simple logging to your application,
you'll need two things:
The [log] crate (this contains macros named after the log level)
and an _adapter_ that actually writes the log output somewhere useful.
Having the ability to use log adapters is very flexible:
You can, for example, use them to write logs not only to the terminal
but also to [syslog], or to a central log server.

[syslog]: https://en.wikipedia.org/wiki/Syslog

Since we are right now only concerned with writing a CLI application,
an easy adapter to use is [env_logger].
It's called "env" logger because you can
use an environment variable to specify which parts of your application
you want to log
(and at which level you want to log them).
It will prefix your log messages with a timestamp
and the module where the log messages come from.
Since libraries can also use `log`,
you easily configure their log output, too.

[log]: https://crates.io/crates/log
[env_logger]: https://crates.io/crates/env_logger

Here's a quick example:

```rust,ignore
{{#include output-log.rs}}
```

Assuming you have this file as `src/bin/output-log.rs`,
on Linux and macOS, you can run it like this:
```console
$ env RUST_LOG=info cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

In Windows PowerShell, you can run it like this:
```console
$ $env:RUST_LOG="info"
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

In Windows CMD, you can run it like this:
```console
$ set RUST_LOG=info
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

`RUST_LOG` is the name of the environment variable
you can use to set your log settings.
`env_logger` also contains a builder
so you can programmatically adjust these settings,
and, for example, also show _info_ level messages by default.

There are a lot of alternative logging adapters out there,
and also alternatives or extensions to `log`.
If you know your application will have a lot to log,
make sure to review them,
and make your users' life easier.

<aside>

**貼士:**
Experience has shown that even mildly useful CLI programs can end up being used for years to come.
(Especially if they were meant as a temporary solution.)
If your application doesn't work
and someone (e.g., you, in the future) needs to figure out why,
being able to pass `--verbose` to get additional log output
can make the difference between minutes and hours of debugging.
The [clap-verbosity-flag] crate contains a quick way
to add a `--verbose` to a project using `clap`.

[clap-verbosity-flag]: https://crates.io/crates/clap-verbosity-flag

</aside>
