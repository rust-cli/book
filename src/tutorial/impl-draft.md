## First implementation of `grrs`

Right, now that we have our input data,
we can start to write our actual tool.
Let’s start by opening the file:

```rust
let content = std::fs::read_to_string(&args.path)?;
```

*Aside:* If the File can’t be read, the `?` will propagate the error and stop the function.

Now, let’s iterate over the lines and print each one that contains our pattern:

```rust
for line in content.lines() {
    if line.contains(args.pattern) {
        println!("{}", line);
    }
}
```

Give it a try: `cargo run -- main src/main.rs` should work now!

*Aside:* This is not the most performant implementation, and will read the whole file into memory.
Feel free to optimize it!
(One idea might be to use a `[BufReader](https://doc.rust-lang.org/1.27.0/std/io/struct.BufReader.html)` instead of `read_to_string()`.)
