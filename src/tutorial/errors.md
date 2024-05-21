# 合適的回饋錯誤

我們都無能為力，只能接受會發生錯誤的事實。

與許多其他語言相比，很難不去注意和應對這個現實。

在使用 Rust 時：既然沒有例外，所有可能的錯誤狀態通常都編碼在函數的傳回型別中。

## 結果

像 [`read_to_string`] 這樣的函數不會回傳字串。
相反的，它會傳回一個 [`Result`]，裡面包含一個“字串”或某種其它型別的錯誤
（在本例子為[`std::io::Error`]）。

[`read_to_string`]: https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.39.0/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

那麼如何得知是哪一種型別呢？ 

因為 `Result` 也是 `enum` 型別， 
可以使用 `match` 去檢查裡面是哪一種變體：

```rust,no_run
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**說明:**
不確定列舉 (`enums`)是什麼或它們在 Rust 中如何工作？
[查看 Rust 官方手冊的這一章節](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html)
好以跟上進度。

</aside>

## 展開

現在，我們可以存取文件的內容，但在 `match` 程式碼區塊後無法對它做任何事情。 

因此，我們需要以某種方式處理錯誤的情況。 
因難點在於， `match` 程式碼區塊的所有分支都會回傳一個相同的型別。 

但有一個巧妙的技巧來解決這個問題：

```rust,no_run
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

我們可以在 `match` 程式碼區塊後使用 `content`。 
如果 `result` 是個錯誤， 字串就不存在。 
但好家在，程式會在我們使用 `content` 之前就會自行退出了。

這種做法看起來有些極端，卻是十分實用的。 
如果你的程式需要讀取一個文件， 並且在文件不存在時無法執行任何操作，那麼退出是十分合理、有效的選擇。
在 `Result` 中還有一個快捷方法，叫做 `unwrap`：

```rust,no_run
let content = std::fs::read_to_string("test.txt").unwrap();
```

## 無須 panic

當然，退出程式並非處理錯誤的唯一辦法。 

除 `panic!`之外，實作 `return` 也很簡單：

```rust,no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
# Ok(())
# }
```

然而，這改變了我們函數的回傳值型別。 
實際上，一直以來我們的範例都隱藏了一些東西： 函數的簽名（或說回傳值型別）。 
在最後的含有 `return` 的範例中，它變得很重要了。
下面是 _完整_ 的範例：

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())
}
```

我們的回傳值型別是 `Result`！ 
這也就是為什麼我們可以在 `match` 的第二個分支寫 `return Err(error);`。 
看到最下面的 Ok(()) 是什麼嗎？ 
它是函數的預設回傳值， 意思為「結果沒問題，沒有內容」。

<aside>

**說明:**
為什麼不寫成 `return Ok(())`？ 
當然這樣寫也完全沒問題。 
在 Rust 中，任何程式碼區塊中的最後的表達式即為其回傳值，因此習慣上省略了 `return`。

</aside>

## 問題(!)標記

如同呼叫 `.unwrap()` 相當於 `match` 中快捷設定錯誤分支中 `panic!`，
我們還有另一個快速的呼叫使得在 `match` 的錯誤分支中直接回傳: `?` 。 

是的，就是這個問號(`?`)。 
你可以在 `Result` 型別後面加上這個運算符號， 
Rust 在內部將會展開產生類似我們剛寫的 match 程式碼區塊。

試試看：

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

非常簡潔！

<aside>

**說明:**
這裡還發生了一些事情，但我們不需要去理解就能使用它。 
例如，在我們的 `main` 函數中，錯誤型別是 `Box<dyn std::error::Error>`。 
但我們在 `read_to_string` 中回傳的卻是 [`std::io::Error`]。 
它能正常工作是因為 `?` 將程式碼擴充為 `converts` 錯誤型別。

`Box<dyn std::error::Error>` 同樣是個很有意思的型別。 
它是一個包含 _任何_ 實作了標準 [`Error`][`std::error::Error`] 特徵型別的 `Box`， 
所以我們可以在所有傳回 `Result` 的一般函式中使用 `?`。

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## 提供內容

在 `main` 函數中使用 `?` 來取得錯誤，可以正常工作，但它有一些不足之處。 
例如：
若使用 `std::fs::read_to_string("test.txt")?` 時，`test.txt` 檔案不存在，
你會得到以下錯誤訊息：

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

在這裡你的程式碼裡沒有包含檔名，
這會讓確認是哪個檔案 `NotFound` 變得很麻煩。
但我們有許多種辦法可以改進它。

例如，我們可以建立一個自己的錯誤型別，
然後使用它去產生自訂的錯誤訊息：

```rust,ignore
{{#include errors-custom.rs}}
```

現在，
運行它將會得到我們剛才自訂的錯誤訊息：

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

儘管不是很完美，
但我們稍後可以輕鬆地為我們的型別除錯輸出。

這種模式實際上很常見。
但它有一個問題：
我們無法儲存原始錯誤，僅只能其輸出的字串來表示形式。
常用的 [`anyhow`] 函式庫對此有一個巧妙的解決方案：
類似於我們的 `CustomError` 型別，
它的 [`Context`] 特徵可用於新增描述。
此外，它還保留了原始錯誤，
因此我們會得到一串( `chain` )錯誤訊息，同時指出根本原因。

[`anyhow`]: https://docs.rs/anyhow
[`Context`]: https://docs.rs/anyhow/1.0/anyhow/trait.Context.html

讓我們先在 `Cargo.toml` 檔案中的 `[dependencies]` 欄位中新增上 `anyhow = "1.0"`。

完整的範例將如下所示：

```rust,ignore
{{#include errors-exit.rs}}
```

這將會輸出一個錯誤：

```text
Error: could not read file `test.txt`

Caused by:
    No such file or directory (os error 2)
```

## 總結

你的程式碼現在看起來應該是這樣的：

```rust,ignore
{{#include errors-impl.rs}}
```
