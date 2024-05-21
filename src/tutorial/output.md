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
這是預設的佔位符號型別， 它嘗試以人類可讀的方式來輸出給定的參數的值。 
對於數值和字串，這會很好用， 
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

如果你想在偵錯和記錄 中輸出自己建置的型別，
大部分情況下你可以在型別定義上新增 `#[derive(Debug)]` 屬性。

<aside>

**說明:**

"使用者友善(User-friendly)" 的輸出是使用 [`Display`] 特性完成的，
除錯輸出（適用於開發者）使用 [`Debug`] 特徵。
您可以在 `println!` 中找到更多關於可以使用的語法的相關資訊。
在[`std::fmt`模組的文件][std::fmt]中。

[`Display`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Display.html
[`Debug`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Debug.html
[std::fmt]: https://doc.rust-lang.org/1.39.0/std/fmt/index.html

</aside>

## 輸出錯誤

輸出錯誤的部分應透過 stderr 完成， 
以便使用者和其它工具更方便的地將輸出通過管道傳輸到文件或更多的工具中。

<aside>

**說明:**
在大部分作業系統中，
一個程式可以將輸出寫入至兩個串流中，`stdout` 和 `stderr`。 
`stdout` 用於程式的實際輸出，而 `stderr` 可將錯誤或其它資訊與 `stdout` 分開。 
這樣，
正確輸出的部分可以儲存到檔案或管道傳輸到其它程式中，同時將錯誤顯示給使用者。

</aside>

在 Rust 中，
使用 `println!` 和 `eprintln!`，
前者對應 `stdout` 和後者對應 `stderr`。

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

<aside>

**小心**: 

輸出 [跳脫碼][escape codes] 可能很危險，
會導致使用者的終端變成奇怪的狀態。 
手動輸出至終端時請務必小心使用！

[escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

當你處理原始[跳脫碼][escape codes]時，
最好使用像 `ansi_term` 這樣的 `crate`， 
以便你（和你程式的使用者）更加放心。

</aside>

## 關於輸出至終端的效能說明

輸出到終端時出奇的慢！ 
如果你在迴圈中呼叫 `println!` 之類的東西， 
它很容易成為其它運行速度快的程式的瓶頸。 
你可以做兩件事來為它提升速度。

首先，
你需要盡量減少實際「刷新」到終端的寫入次數。
_每次_ 呼叫 `println!` 時，
它都會告訴系統刷新到終端，
因為列印每個新行是很常見的。 
如果你不需要如此， 
你可以使用 [`BufWriter`] 來包裝一下 `stdout` 的句柄，
它的預設快取為 8 kB。 
( 當你想立即輸出至終端時，請在 `BufWriter` 上呼叫 `.flush()` 即可。)

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
```

其次，
為 `stdout`（或 `stderr` ）申請一把鎖並使用 `writeln!` 來直接輸出至終端會很有用。 
它會阻止系統不停地鎖定並解鎖 `stdout`。

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = stdout.lock(); // acquire a lock on it
writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
```

你也可以結合且使用這兩種方式。

[`BufWriter`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufWriter.html

## 顯示進度條

有些 CLI 程式的運作時間很長，
會花費幾分鐘甚至數小時。 
如果你在編寫這種程式，
你可能想要向使用者展示，其程式正在正常運作中。 
因此，你需要輸出有用的狀態更新訊息，
最好是使用易於使用的方式來進行輸出。


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

## 記錄檔

為了更方便了解我們的程式做了什麼，
我們需要為它加入一些記錄檔的相關語句，這很簡單。 
但在長時間後，例如半年後再執行這個程式時，
記錄檔就變得非常有用了。 
在某些方面來說， 
記錄的使用方法與 `println` 一樣類似，
只是它可以指定訊息的重要性（層級）。 
通常可以使用的層級包括 _error_ , _warn_, _info_ , _debug_ , 和 _trace_ 
（ _error_ 優先順序最高， _trace_ 最低）。

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
