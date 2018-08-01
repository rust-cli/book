# First implementation of `grrs`

Right, now that we have our input data,
we can start to write our actual tool.
We'll only work with `src/main.rs` for now.

<aside class="shortcut">

**Shortcut:**
If you skipped the CLI argument parsing chapter,
you can put

```rust
let path = "test.txt";
let pattern = "foo";
```

at the top of your file
and replace `args.path` with `path`
and `args.pattern` with `pattern`
in the example code below.

</aside>

Let’s start by opening the file we got:

```rust
let content = std::fs::read_to_string(&args.path)?;
```

<aside>

**Aside:**
If the file can’t be read,
the question mark operator (`?`)
will propagate the error and return from the function.
It basically turns the line into this:

```rust
let content = match std::fs::read_to_string(&args.path) {
    Ok(value) => value,
    Err(error) => return error,
};
```

Read more about this in the 
[error handling chapter of the Rust book](https://doc.rust-lang.org/1.27.2/book/second-edition/ch09-00-error-handling.html).

</aside>

Now, let’s iterate over the lines
and print each one that contains our pattern:

```rust
for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
```

Give it a try: `cargo run -- main src/main.rs` should work now!

<aside>

**Aside:**
This is probably not the best implementation,
as it will read the whole file into memory
-- however large the file may be!
Feel free to optimize it!
(One idea might be to use a [`BufReader`](https://doc.rust-lang.org/1.27.0/std/io/struct.BufReader.html)
instead of `read_to_string()`.)

</aside>
