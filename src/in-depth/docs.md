# 為你的 CLI 程式產生文件 

CLI 程式的文件 通常會包括指令中的 `--help` 部分和一個手冊（`man`）頁面。

兩者都可以自動產生
當使用 [`clap`](https://crates.io/crates/clap) 時，
會透過 [`clap_mangen`](https://crates.io/crates/clap_mangen) crate。

```rust,ignore
#[derive(Parser)]
pub struct Head {
    /// file to load
    pub file: PathBuf,
    /// how many lines to print
    #[arg(short = "n", default_value = "5")]
    pub count: usize,
}
```

其次，您需要使用 `build.rs`在編譯時,
請根據您的應用程式在程式碼中的定義而產生手冊文件。

在這裡你要留意幾件事（例如如何打包你的程式）， 
但現在我們只是簡單地把 `man` 檔案放到我們的 `src` 同等級目錄。

```rust,ignore
use clap::CommandFactory;

#[path="src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?);
    let cmd = cli::Head::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("head.1"), buffer)?;

    Ok(())
}
```

When you now compile your application
there will be a `head.1` file
in your project directory.

If you open that in `man`
you'll be able to admire your free documentation.
如果你在 `man` 中打開它，則你可閱讀文件
