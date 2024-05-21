# 退出狀態碼

A program doesn't always succeed.
And when an error occurs,
you should make sure to emit the necessary information correctly.
In addition to
[telling the user about errors](human-communication.html),
on most systems,
when a process exits,
it also emits an exit code
(an integer between 0 and 255 is compatible with most platforms).
You should try to emit the correct code
for your program's state.
For example,
in the ideal case when your program succeeds,
it should exit with `0`.

一個計劃並不總是成功。
並且當出現錯誤時，您應該確保正確發出必要的資訊。
另外[告訴使用者錯誤]( human-communication.html),
在大多數系統上，當進程退出時，它還發出退出代碼（0 到 255 之間的整數與大多數平台相容）。
您應該嘗試發出正確的程式碼
對於你的程式的狀態。
例如，在您的計劃成功的理想情況下，它應該以“0”退出。

When an error occurs, it gets a bit more complicated, though.
In the wild,
many tools exit with `1` when a common failure occurs.
Currently, Rust sets an exit code of `101` when the process panicked.
Beyond that, people have done many things in their programs.

So, what to do?
The BSD ecosystem has collected a common definition for their exit codes
(you can find them [here][`sysexits.h`]).
The Rust library [`exitcode`] provides these same codes,
ready to be used in your application.
Please see its API documentation for the possible values to use.

After you add the `exitcode` dependency to your `Cargo.toml`,
you can use it like this:

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
