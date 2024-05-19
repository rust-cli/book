# 解析命令列參數

我們的 CLI 工具的呼叫方法應該如下：

```console
$ grrs foobar test.txt
```

我們希望此程式將尋找 `test.txt` 並列印出包含 `foobar` 的行。 

但我們如何取得這兩個值呢？

在命令列中，程式名稱中後面的文字通常被稱為「命令列參數(command-line arguments)」或「命令列標籤(command-line flags)」（尤其是當他們看起來像 `--this`）。 

作業系統通常會將它們識別為字串列表——簡單的說，以空格分隔。

有很多方法可以識別這些參數並解析，使它們變得更易於使用。 

同時也需要告訴用戶， 程式需要哪些參數及對應的格式是什麼。

## 取得參數

標準庫中提供的 [`std::env::args()`] 方法，提供了運行時給定參數的[疊代器(iterator)]。 

首先，第一項（索引 `0` ）是程式名稱（如 : `grrs`），後面部分才是使用者給定的參數。

[`std::env::args()`]: https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
[疊代器(iterator)]: https://doc.rust-lang.org/1.39.0/std/iter/index.html

以此方法取得原始參數就是這麼簡單（在 `src/main.rs` 的 `fn main()` 函數中）：

```rust,ignore
{{#include cli-args-vars.rs}}
```

我們可以使用 `cargo run` 來運行它，透過在 `--` 之後寫入參數來傳遞參數：

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
pattern: "some-pattern", path: "some-file"
```

## CLI 參數的資料類型

與其將它們視為單純的一堆文本，不如將 CLI 參數看成程式輸入的自訂的資料類型。

注意 `grrs foobar test.txt`:
這裡有兩個參數，首先是 `pttern`（查看的字串）， 然後才是 `path`（尋找的檔案路徑）。

關於他們，我們還能說些什麼呢？

首先，這兩個參數都是程式所必須的，因為我們並未提供預設值， 所以使用者需要在使用此程式時提供這兩個參數。 

此外，關於參數的類型：pattern 應該是字串；第二個參數則應是檔案的路徑。

在Rust 中，根據所處理的資料去建立程式是很常見的， 因此這種看待參數的方法對我們接下來的工作很有幫助。

讓我們以此開始（在 `src/main.rs`，`fn main( ) {` 之前 ）：

```rust,ignore
{{#include cli-args-struct.rs:1:4}}
```

This defines a new structure (a [`struct`])
that has two fields to store data in: `pattern`, and `path`.

[`struct`]: https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html

<aside>

**Note:**
[`PathBuf`] is like a [`String`] but for file system paths that work cross-platform.

[`PathBuf`]: https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.39.0/std/string/struct.String.html

</aside>

Now, we still need to get the actual arguments our program got into this form.
One option would be to manually parse the list of strings we get from the operating system
and build the structure ourselves.
It would look something like this:

```rust,ignore
{{#include cli-args-struct.rs:6:16}}
```

This works, but it's not very convenient.
How would you deal with the requirement to support
`--pattern="foo"` or `--pattern "foo"`?
How would you implement `--help`?

## Parsing CLI arguments with Clap

A much nicer way is to use one of the many available libraries.
The most popular library for parsing command-line arguments
is called [`clap`].
It has all the functionality you'd expect,
including support for sub-commands, [shell completions], and great help messages.

[`clap`]: https://docs.rs/clap/
[shell completions]: https://docs.rs/clap_complete/

Let's first import `clap` by adding
`clap = { version = "4.0", features = ["derive"] }` to the `[dependencies]` section
of our `Cargo.toml` file.

Now, we can write `use clap::Parser;` in our code,
and add `#[derive(Parser)]` right above our `struct Cli`.
Let's also write some documentation comments along the way.

It’ll look like this (in file `src/main.rs`, before `fn main() {`):

```rust,ignore
{{#include cli-args-clap.rs:1:10}}
```

<aside class="node">

**Note:**
There are a lot of custom attributes you can add to fields.
For example,
to say you want to use this field for the argument after `-o` or `--output`,
you'd add `#[arg(short = 'o', long = "output")]`.
For more information,
see the [clap documentation][`clap`].

</aside>

Right below the `Cli` struct our template contains its `main` function.
When the program starts, it will call this function:

```rust,ignore
{{#include cli-args-clap.rs:12:16}}
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
The `parse` method is meant to be used in your `main` function.
When it fails,
it will print out an error or help message
and immediately exit the program.
Don't use it in other places!

</aside>

## Wrapping up

Your code should now look like:

```rust,ignore
{{#include cli-args-clap.rs}}
```

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

Running it passing arguments:

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
pattern: "some-pattern", path: "some-file"
```

The output demonstrates that our program successfully
parsed the arguments into the `Cli` struct.
