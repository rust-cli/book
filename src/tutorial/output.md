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
它希望您撰寫一個字串文字作為第一個參數，包含將要填入的佔位符號
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
因為輸出每個新行是很常見的。 
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
如果你在撰寫這種程式，
你可能需要向使用者展示，其程式正在正常運作中。 
因此，你需要輸出有用的狀態更新訊息，
最好是使用易於使用的方式來進行輸出。

你可以使用 [indicatif] crate 來為你的程式加入進度條，
這是一個快速的例子：

```rust,ignore
{{#include output-progressbar.rs:1:9}}
```

細節部分可試著查看 indicatif 的 [相關文件][indicatif docs] 和 [範例][indicatif examples] 。

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

只需這兩樣東西，你就可以為你的程式加入簡單的記錄功能：
Log create（其中包含以記錄等級命名的巨集）和一個 _轉接器_， 
它會將記錄寫到有用的地方。 
記錄轉接器的使用是十分靈活的： 
例如，你可以不僅將記錄輸出至終端，同時也可寫進 [syslog] 或其它記錄伺服器。

[syslog]: https://en.wikipedia.org/wiki/Syslog

因為我們現在最關心的是寫一個 CLI 程式，
所以選取一個易於使用的轉接器 [env_logger]。 
它之所以叫 `env` 記錄器，
因為你可以使用環境變數來指定程式中哪一部分需要記錄和記錄哪一個等級。 
它會在你的記錄資訊前加上時間戳記及所在模組資訊。 
由於函式庫也可以使用 `log`，你也可以輕鬆地配置它們的記錄輸出。

[log]: https://crates.io/crates/log
[env_logger]: https://crates.io/crates/env_logger

這是一個簡單的範例:

```rust,ignore
{{#include output-log.rs}}
```

如果你有 `src/bin/output-log.rs` 這個文件，
在 Linux 和 MacOS 上，您可以像這樣執行它：
```console
$ env RUST_LOG=info cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

在 Windows PowerShell 中，您可以像這樣執行它：
```console
$ $env:RUST_LOG="info"
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

在 Windows CMD(命令提示字元) 中，您可以像這樣執行它：
```console
$ set RUST_LOG=info
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

`RUST_LOG` 是設定 log 的環境變數名稱，
您可以使用它來設定記錄檔設定。
`env_logger` 還包含一個建置器，
因此你可以以程式設計的方式調整這些設置， 
例如，預設顯示 _info_ 等級的記錄。

有很多替代的記錄器，
以及 `log` 的替代品或擴充功能。
如果您知道您的應用程式將有很多需要記錄的內容，
請確保檢查它們，
並讓您的使用者的生活更輕鬆。

<aside>

**貼士:**
經驗表明，即使是用處不大的 CLI 程式，最終也會被使用多年。
( 特別是如果它們只是作為臨時解決方案的話 ）。
如果您的應用程式無法運行時，
而有人（例如將來的你）需要找出原因、
能夠透過 `--verbose` 獲得額外的記錄輸出
可以讓偵錯工作在幾分鐘和幾小時之間取得顯著效果。
[clap-verbosity-flag] create 包含了快速的
在使用 `clap` 的專案中新增 `--verbose` 的快速方法。

[clap-verbosity-flag]: https://crates.io/crates/clap-verbosity-flag

</aside>
