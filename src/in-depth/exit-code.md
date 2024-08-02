# 退出狀態碼

程式並不總是成功的。
當錯誤發生時，你應該確保正確地發出必要的資訊，
除了[告訴使用者錯誤訊息](human-communication.html)，
在大多數系統中，當行程退出時也會發出一個退出代碼
( 一個介於 0 和 255 之間的整數，與大多數平台相容 ）。
您應盡量根據程式的狀態去制定狀態碼。
例如： 在程式成功運行的理想情況下，應該以 `0` 退出。

所以，要如何去做呢？ 
BSD 生態系統為其退出碼做了一個通用的定義 （你可以在[這裡][`sysexits.h`]找到它們）。 
Rust 的 [`exitcode`] 函式庫也提供了一樣的程式碼，
而且你可在你的程式中使用。 
請參閱其 API 文件以了解其用法。

當你在你的 `Cargo.toml` 中加入 `exitcode` 依賴後，
你可以這樣使用：

```rust,ignore
fn main() {
    // ...actual work...
    match result {
        Ok(_) => {
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        Err(CustomError::CantReadConfig(e)) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::CONFIG);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
```


[`exitcode`]: https://crates.io/crates/exitcode
[`sysexits.h`]: https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
