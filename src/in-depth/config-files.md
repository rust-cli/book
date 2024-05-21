# 使用配置檔

Dealing with configurations can be annoying
especially if you support multiple operating systems
which all have their own places
for short- and long-term files.

處理配置可能很煩人
特別是如果您支援多個作業系統
都有自己的位置
適用於短期和長期文件。

對此有許多解決方案，有些比其他的層次更低。

The easiest crate to use for this is [`confy`].
It asks you for the name of your application
and requires you to specify the config layout
via a `struct` (that is `Serialize`, `Deserialize`)
and it will figure out the rest!

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
2. CLI-args + 多個配置 + 環境變數
3. [`configure`] 可以完成這一切嗎？ 周圍有漂亮的包裝嗎？

</aside>

[`configure`]: https://docs.rs/configure/0.1.1/configure/
