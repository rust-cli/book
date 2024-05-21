# _grrs_ 的首次運行

在完成命令列參數章節後，我們學到了如何取得輸入參數，和我們可以開始準備實作寫出工具了。 

目前我們的 `main` 函數中僅有一行：

```rust,ignore
{{#include impl-draft.rs:13:13}}
```

（我們刪除暫時放在那裡的 `println` 語句，得以證明我們的程式如預期般運作。）

讓我們先開啟我們得到的文件。

```rust,ignore
{{#include impl-draft.rs:14:14}}
```

<aside>

**說明:**
看到 [`.expect`] 方法了吧？ 
這是一個快捷功能，當無法讀取到參數值時 （這裡是指輸入的檔案）會立即退出程式。
它還並不完美，在下一章節 [合適的回饋錯誤][Nicer error reporting] 中，
我們將探究如何改進它。

[`.expect`]: https://doc.rust-lang.org/1.39.0/std/result/enum.Result.html#method.expect
[Nicer error reporting]:./errors.html

</aside>

現在，讓我們疊代一下這些行並輸出包含每一個我們的模式：

```rust,ignore
{{#include impl-draft.rs:16:20}}
```

## 總結

你的程式碼現在看起來應該是這樣的：

```rust,ignore
{{#include impl-draft.rs}}
```

來試試: `cargo run -- main src/main.rs` 是否能運行！

<aside class="exercise">

**供讀者練習:**
這並非最好的實作：
無論檔案有多大，它將整個檔案讀取到記憶體中了。 
去尋找最佳化的方法吧！ 
（有一種想法就是可能用 [`BufReader`] 而不是 `read_to_string()` 。）

[`BufReader`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html

</aside>
