# 在15分鐘內建立一個命令列應用程式

本教學將指導您建立CLI（命令列介面）應用程式

在[Rust]中，只需大約需要十五分鐘，就能得到可以運行的程式（大約第 1.3 節以後）。

之後，我們只需持續調整我們的程式，直到它可以被當作一個工具來打包發布。

[Rust]: https://rust-lang.org/

你將學到如何開始所需的所有基本知識，以及如何去尋找更多有用資訊。
當然，你可隨意跳過目前不需要了解的章節，或之後再翻回查看。

<aside>

**先決條件:**
本教學並不能取代程式設計的一般性介紹，你最好了解、熟悉一些常見的概念。
同樣，你應該能熟練的使用命令列或終端。如果你會使用其它的語言，那麼這對於你接觸和學習 Rust 會是極大幫助。

**尋求協助:**
如果你在任何時候對所使用的功能感到不解或困惑，請查閱 Rust 提供的官方文件 ，首先是這本《The Rust Programming Language》。 
一般安裝 Rust 時也會安裝它 （`rustup doc`），或者你也可以在線查看 [doc.rust-lang.org]。
也非常歡迎你來社區提問——Rust 社區以友好和樂於助人著稱。 在 [社群頁面] 上你可以看到人們關於 Rust 的討論的清單。 You are also very welcome to ask questions –

[doc.rust-lang.org]: https://doc.rust-lang.org
[社群頁面]: https://www.rust-lang.org/community

</aside>

你想要寫一個什麼樣的專案呢？ 

不如我們先從一個簡單的開始：讓我們寫一個簡單的 `grep`。 

我們給這個工具一個字串和一個檔案路徑，它將列印出每個包含所查字串的行。 不如就叫它 `grrs` 吧（發音“grass”）。


最後，我們想讓它像這樣運行：
```console
$ cat test.txt
foo: 10
bar: 20
baz: 30
$ grrs foo test.txt
foo: 10
$ grrs --help
[some help text explaining the available options]
```

<aside class="note">

**說明:**

本書是使用 [Rust 2018] 所寫的。
同時，這些程式碼範例同樣適用於 Rust 2015， 只需做一點點調整，例如新增 `extern crate foo;`。

確保你執行的是 Rust 1.31.0（或更高版本），同時在你的 `Cargo.toml` 檔案中， 在 `[package]` 段落加上 `edition = "2018"`。

[Rust 2018]: https://doc.rust-lang.org/edition-guide/index.html

</aside>
