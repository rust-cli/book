# 專案初始化設置

如果你還沒有在你的電腦上[安裝 Rust]（應該只需要幾分鐘就能完成）。

然後，請打開終端機並切換到你的工作目錄，程式原始碼將會放置在這裡。

[安裝 Rust]: https://www.rust-lang.org/tools/install

請開始在你想建立專案的目錄下執行`cargo new grrs`。

如果你查看新建立的 `grrs` 目錄，你會發現 Rust 專案的預設設定：

- `Cargo.toml` 裡包含了我們專案所有的中繼資料，包括我們使用依賴/外部函式庫列表。
- `src/main.rs`  是我們程式的二進制入口檔案（主程式）。

如果可以在`grrs`目錄下執行`cargo run`並得到一個 `Hello, World!`，那你就已經準備好了。

## 它可能會是什麼樣子

```console
$ cargo new grrs
     Created binary (application) `grrs` package
$ cd grrs/
$ cargo run
   Compiling grrs v0.1.0 (/Users/pascal/code/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/grrs`
Hello, world!
```
