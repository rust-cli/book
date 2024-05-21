# 解析命令列參數

我們的 CLI 工具的呼叫方法應該如下：

```console
$ grrs foobar test.txt
```

我們希望此程式將尋找 `test.txt` 並列印出包含 `foobar` 的行。 

但我們如何取得這兩個值呢？

在命令列中，程式名稱中後面的文字通常被稱為「命令列參數(command-line arguments)」或「命令列標籤(command-line flags)」（尤其是當他們看起來像 `--this`）。 

作業系統通常會將它們識別為字串列表 --- 簡單的說，以空格分隔。

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

## CLI 參數的資料型別

與其將它們視為單純的一堆文字，不如將 CLI 參數看成程式輸入的自訂的資料型別。

注意 `grrs foobar test.txt`:
這裡有兩個參數，首先是 `pttern`（查看的字串）， 然後才是 `path`（尋找的檔案路徑）。

關於他們，我們還能說些什麼呢？

首先，這兩個參數都是程式所必須的，因為我們並未提供預設值， 所以使用者需要在使用此程式時提供這兩個參數。 

此外，關於參數的型別：pattern 應該是字串；第二個參數則應是檔案的路徑。

在Rust 中，根據所處理的資料而去建立程式是很常見的， 因此這種看待參數的方法對我們接下來的工作很有幫助。

讓我們在這開始（在檔案 `src/main.rs`，`fn main( ) {` 之前 ）：

```rust,ignore
{{#include cli-args-struct.rs:1:4}}
```

這定義了一個新的結構體（a [`struct`]）
它有兩個欄位來儲存資料： `patern` 和 `path` 。

[`struct`]: https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html

<aside>

**說明:**
[`PathBuf`] 是可跨平台使用的系統路徑型別，特性類似 [字串][`String`]。

[`PathBuf`]: https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.39.0/std/string/struct.String.html

</aside>

現在，我們仍然需要取得我們的程式進入這種形式的實際參數。

有一種做法就是我們可以手動解析從作業系統上取得的參數列表並以此產生一個結構體。

就有點像是這樣：

```rust,ignore
{{#include cli-args-struct.rs:6:16}}
```

這種方式能正常運作，但用起來卻不是很方便。 
如何去支援像 `--pattern="foo"` 或 `--pattern "foo"` 這種參數輸入？ 
又如何去實現 `--help`？

## 使用 Clap 解析 CLI 參數

使用現成的函式庫來實現參數的解析，這是更明智的選擇。 
[`clap`] 是目前最受歡迎的解析命令列參數的函式庫。 
它提供了所有你需要的功能， 如子指令、[自動補全][shell completions] 和完善的幫助資訊。

[`clap`]: https://docs.rs/clap/
[shell completions]: https://docs.rs/clap_complete/

首先，我們需要在 `Cargo.toml` 檔案的 `[dependencies]` 欄位裡加入上 `clap = { version = "4.0", features = ["derive"] }` 來匯入 `clap`。

現在，我們可以在程式碼中加入`use clap::Parser;`,
和在先前建立的 `struct Cli` 的正上方加上 `#[derive(Parser)]`。

我們也可以在過程中編寫一些文件註解。

讓我們在這開始（在檔案 `src/main.rs`，`fn main( ) {` 之前 ）：

```rust,ignore
{{#include cli-args-clap.rs:1:10}}
```

<aside class="node">

**說明:**
你可以將許多自訂屬性加入到欄位中。 
例如，
如果你想將 `-o` 或 `--output` 後的參數解析為某個字段，可在字段上方添加上 `#[structopt(short = "o", long = "output")]`。 
如需更多屬性設置，請查看 [clap documentation][`clap`]。

</aside>

在本範例中，我們的 `Cli` 結構體下方即是 main 函數。 
當開始執行程式時，就會呼叫這個函數：

```rust,ignore
{{#include cli-args-clap.rs:12:16}}
```

這將嘗試解析參數並儲存到 `Cli` 結構體中。

但如果解析失敗會怎樣？ 
這就是使用此方法的美妙之處：Clap 知道它需要什麼字段， 及所需字段的型別。 
它可以自動產生一個不錯的 `--help` 訊息， 並會依錯誤給予一些建議－輸入的參數應該是 `--output` 而你輸入的是 `--putput`。

<aside class="note">

**說明:**
`parse` 方法應該在 `main` 函數中使用。
當它失敗時，
它將輸出錯誤或幫助訊息並立即退出該程式。
請勿在其他地方使用！

</aside>

## 總結

你的程式碼現在看起來應該是這樣的：

```rust,ignore
{{#include cli-args-clap.rs}}
```

在無指定參數運作時：

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

如果使用傳遞參數:

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
pattern: "some-pattern", path: "some-file"
```

該輸出表示我們的程式成功將參數解析為 `Cli` 結構。
