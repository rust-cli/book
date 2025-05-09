# Nicer error reporting

We all can do nothing but accept the fact that errors will occur.
In contrast to many other languages,
it's very hard not to notice and deal with this reality
when using Rust because it doesn't have exceptions.
All possible error states are often encoded in the return types of functions.

## Results

A function like [`read_to_string`] doesn't return a string.
Instead, it returns a [`Result`]
that contains either
a `String`
or an error of some type.
In this case, [`std::io::Error`].

[`read_to_string`]: https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.39.0/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

How do you know which it is?
Since `Result` is an `enum`,
you can use `match` to check which variant it is:

```rust,no_run
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**Note:**
Not sure what enums are or how they work in Rust?
[Check out this chapter of the Rust book](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html)
to get up to speed.

</aside>

## Unwrapping

Now, we were able to access the content of the file,
but we can't really do anything with it after the `match` block.
For this, we'll need to deal with the error case.
While it's a challenge that all arms of a `match` block need to return something of the same type,
there's a neat trick to get around that:

```rust,no_run
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

We can use the String in `content` after the match block, but
if `result` were an error, the String wouldn't exist.
That's fine because the program would exit before it ever reached a point where we use `content`.

This may seem drastic,
but it's very convenient.
If your program needs to read that file and can't do anything if the file doesn't exist,
exiting is a valid strategy.
There's even a shortcut method on [`Result`] called `unwrap`:

```rust,no_run
let content = std::fs::read_to_string("test.txt").unwrap();
```

## No need to panic

Of course, aborting the program is not the only way to deal with errors.
Instead of using `panic!`, we can just use `return`:

```rust,no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
# Ok(())
# }
```

However, this changes the return type in our function.
There was something hidden in our examples all this time:
The function signature this code lives in.
And in this last example with `return`,
it becomes important.
Here's the _full_ example:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())
}
```

Our return type is a `Result`!
This is why we can write `return Err(error);` in the second match arm.
See how there is an `Ok(())` at the bottom?
It's the default return value of the function and means:
"Result is okay, and has no content".

<aside>

**Note:**
Why is this not written as `return Ok(());`?
It easily could be – this is totally valid as well.
The last expression of any block in Rust is its return value,
and it is customary to omit a needless `return`.

</aside>

## Question Mark

Just like calling `.unwrap()` is a shortcut
for the `match` with `panic!` in the error arm,
we have another shortcut for the `match` that `return`s in the error arm:
`?`.

That's right, a question mark.
You can append this operator to a value of type `Result`,
and Rust will internally expand this to something very similar to
the `match` we just wrote.

Give it a try:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

Very concise!

<aside>

**Note:**
There are a few more things happening here
that are not required to understand to work with this.
For example,
the error type in our `main` function is `Box<dyn std::error::Error>`,
but we've seen above that `read_to_string` returns a [`std::io::Error`].
This works because `?` expands to code that  _converts_ error types.

`Box<dyn std::error::Error>` is also an interesting type.
It's a `Box` that can contain _any_ type
that implements the standard [`Error`][`std::error::Error`] trait.
This means that all errors can be put into this box,
and we can use `?` on all of the usual functions that return a `Result`.

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## Providing Context

The errors you get when using `?` in your `main` function are okay,
but they are not great.
For example,
when you run `std::fs::read_to_string("test.txt")?`
and the file `test.txt` doesn't exist,
you get this output:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

In cases where your code doesn't actually contain the file name,
it would be hard to tell which file was `NotFound`.
There are multiple ways to deal with this.

For one, we can create our own error type
and use that to build a custom error message:

```rust,ignore
{{#include errors-custom.rs}}
```

Running this, we'll get our custom error message:

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

Not very pretty,
but we can adapt the debug output for our type later on.

This pattern is very common.
It has one problem though:
We don't store the original error,
only its string representation.
The popular [`anyhow`] library has a neat solution for that:
Its [`Context`] trait can be used to add a description similar to our `CustomError` type.
Additionally, it keeps the original error,
so we get a "chain" of error messages pointing to the root cause.

[`anyhow`]: https://docs.rs/anyhow
[`Context`]: https://docs.rs/anyhow/1.0/anyhow/trait.Context.html

Let's first import the `anyhow` crate by adding
`anyhow = "1.0"` to the `[dependencies]` section
of our `Cargo.toml` file.

The full example will look like this:

```rust,ignore
{{#include errors-exit.rs}}
```

This will print an error:

```text
Error: could not read file `test.txt`

Caused by:
    No such file or directory (os error 2)
```

## Wrapping up

Your code should now look like:

```rust,ignore
{{#include errors-impl.rs}}
```
