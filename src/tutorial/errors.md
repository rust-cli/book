# 合適的反饋錯誤

我們都無能為力，只能接受錯誤會發生的事實。

與許多其他語言相比，很難不去注意和應對這個現實
使用 Rust 時：
既然沒有例外，所有可能的錯誤狀態通常都編碼在函數的傳回類型中。

## 結果

像 [`read_to_string`] 這樣的函數不回傳字串。
相反的，它會傳回一個 [`Result`]，裡面包含一個“字串”或某種其它類型的錯誤
（在本例中為[`std::io::Error`]）。

[`read_to_string`]: https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.39.0/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

那麼如何得知是哪一種類型呢？ 

因為 `Result` 也是 `enum` 類型， 
可以使用 `match` 去檢查裡面是哪一種變體：

```rust,no_run
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**筆記:**
不確定枚舉(`enums`)是什麼或它們在 Rust 中如何工作？
[查看 Rust 書的這一章](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html)
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

然而，這改變了我們函數的回傳值類型。 
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

我們的回傳值類型是 `Result`！ 
這也就是為什麼我們可以在 `match` 的第二個分支寫 `return Err(error);`。 
看到最下面的 Ok(()) 是什麼嗎？ 
它是函數的預設回傳值， 意思為「結果沒問題，沒有內容」。

<aside>

**筆記:**
Why is this not written as `return Ok(());`?
It easily could be – this is totally valid as well.
The last expression of any block in Rust is its return value,
and it is customary to omit needless `return`s.

</aside>

## 問題標記

Just like calling `.unwrap()` is a shortcut
for the `match` with `panic!` in the error arm,
we have another shortcut for the `match` that `return`s in the error arm:
`?`.

That's right, a question mark.
You can append this operator to a value of type `Result`,
and Rust will internally expand this to something very similar to
the `match` we just wrote.

試一試：

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

非常簡潔！

<aside>

**Note:**
There are a few more things happening here
that are not required to understand to work with this.
For example,
the error type in our `main` function is `Box<dyn std::error::Error>`.
But we've seen above that `read_to_string` returns a [`std::io::Error`].
This works because `?` expands to code that  _converts_ error types.

`Box<dyn std::error::Error>` is also an interesting type.
It's a `Box` that can contain _any_ type
that implements the standard [`Error`][`std::error::Error`] trait.
This means that basically all errors can be put into this box,
so we can use `?` on all of the usual functions that return `Result`s.

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## Providing Context

The errors you get when using `?` in your `main` function are okay,
but they are not great.
For example:
When you run `std::fs::read_to_string("test.txt")?`
but the file `test.txt` doesn't exist,
you get this output:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

In cases where your code doesn't literally contain the file name,
it would be very hard to tell which file was `NotFound`.
There are multiple ways to deal with this.

For example, we can create our own error type,
and then use that to build a custom error message:

```rust,ignore
{{#include errors-custom.rs}}
```

Now,
running this we'll get our custom error message:

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

Not very pretty,
but we can easily adapt the debug output for our type later on.

This pattern is in fact very common.
It has one problem, though:
We don't store the original error,
only its string representation.
The often used [`anyhow`] library has a neat solution for that:
similar to our `CustomError` type,
its [`Context`] trait can be used to add a description.
Additionally, it also keeps the original error,
so we get a "chain" of error messages pointing out the root cause.

[`anyhow`]: https://docs.rs/anyhow
[`Context`]: https://docs.rs/anyhow/1.0/anyhow/trait.Context.html

Let's first import the `anyhow` crate by adding
`anyhow = "1.0"` to the `[dependencies]` section
of our `Cargo.toml` file.

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
