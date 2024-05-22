# 使用配置檔

處理配置可能會很煩人
尤其是在支援多個作業系統的情況下
都有自己的短期和長期文件。

對此有許多解決方案，有些比其他的層次更低。

最容易使用的crate是 [`confy`]。
它會詢問您的應用程式名稱
並請您指定配置佈局 __結構(`struct`)__
（即 __序列化(`Serialize`)__ 、 __反序列化(`Deserialize`)__ ）。
然後剩下的它將解決其他問題！

```rust,ignore
#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

fn main() -> Result<(), io::Error> {
    let cfg: MyConfig = confy::load("my_app")?;
    println!("{:#?}", cfg);
    Ok(())
}
```

這非常容易使用
為此，您當然要放棄可設定性。
但如果你想要一個簡單的配置，
這個crate可能適合你！

[`confy`]: https://docs.rs/confy/0.3.1/confy/

## 配置環境

<aside class="todo">

**TODO**

1. 評估現有的 crate
2. 命令列參數(CLI-args) + 多個配置 + 環境變數
3. [`configure`] 可以完成這一切嗎？ 周圍有漂亮的包裝嗎？

</aside>

[`configure`]: https://docs.rs/configure/0.1.1/configure/
