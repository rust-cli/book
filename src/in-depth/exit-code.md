# Exit codes

A program doesn't always succeed.
And when an error occurs,
you should make sure to emit the necessary information correctly.
In addition to
[telling the user about errors](human-communication.md),
on most systems,
when a process exits,
it also emits an exit code
(an integer between 0 and 255).
You should try to emit the correct code
for your program's state.
For example,
in the ideal case when your program succeeds,
it should exit with `0`.

When an error occurs, it gets a bit more complicated, though.
In the wild,
a lot of tools exit with `1` when a general failure ocurred.
Currently, Rust set and exit code of `101` when the process panicked.
Beyond that, people have done many things in their programs.

So, what to do?
The BSD ecosystem has collected a common definition for their exit codes
in a system-provided header file called [`sysexits.h`]
The Rust library [`exitcode`] provides these same codes,
ready to be used in your application.
Please see its API documentation for the possible values to use.

One way to use it is like this:

```rust
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
