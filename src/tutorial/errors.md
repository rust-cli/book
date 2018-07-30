# Nicer error reporting

We all can do nothing but accept the fact that errors will occur.
And in contrast to many other languages,
it's very hard not to notice and deal with this reality
when using Rust:
As it doesn't have exceptions,
all possible error states are often encoded in the return types of functions.

## Results?

A function like [`read_to_string`] doesn't return a string.
Instead, it returns a [`Result`]
that contains either
a `String`
or an error of some type
(in this case [`std::io::Error`]).

[`read_to_string`]: https://doc.rust-lang.org/1.27.2/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.27.2/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.27.2/std/io/type.Result.html

How do you know which it is?
Since `Result` is an `enum`,
you can use `match` to check which variant it is:

```rust
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**Aside:**
Not sure what enums are or how they work in Rust?
[Check this chapter of the Rust book](https://doc.rust-lang.org/1.27.2/book/second-edition/ch06-00-enums.html)
to get up to speed.

</aside>

Now, we were able to access content of the file,
but we can't really do anything with it after the `match` block.
For this, we'll need to somehow deal with the error case.
The challenge is that all arms of a `match` block need to return something of the same type.
But there's a need trick to get around that:

```rust
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

We can use the String in `content` after the match block.
If `result` were an error, the String wouldn't exist.
But since the program would exit before it ever reached a point where we use `content`,
it's fine.

This may seem drastic,
but it's very convenient.
If your program needs to read that file and can't do anything if the file doesn't exist,
exiting is a valid strategy.
There's even a shortcut method on `Result`s, called `unwrap`:

```rust
let content = std::fs::read_to_string("test.txt").unwrap();
```

<aside class="todo">

**TODO:** `?` operator and returning result in main

</aside>

<aside class="todo">

**TODO:** Replace `?` with `.with_context(|_| format!("could not read file {}", args.path))`

</aside>
