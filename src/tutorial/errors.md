# Nicer error reporting

We all can do nothing but accept the fact that errors will occur.
And in contrast to many other languages,
it's very hard not to notice and deal with this reality
when using Rust:
As it doesn't have exceptions,
all possible error states are often encoded in the return types of functions.

## Results

A function like [`read_to_string`] doesn't return a string.
Instead, it returns a [`Result`]
that contains either
a `String`
or an error of some type
(in this case [`std::io::Error`]).

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

**Aside:**
Not sure what enums are or how they work in Rust?
[Check this chapter of the Rust book](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html)
to get up to speed.

</aside>

## Unwrapping

Now, we were able to access the content of the file,
but we can't really do anything with it after the `match` block.
For this, we'll need to somehow deal with the error case.
The challenge is that all arms of a `match` block need to return something of the same type.
But there's a neat trick to get around that:

```rust,no_run
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

```rust,no_run
let content = std::fs::read_to_string("test.txt").unwrap();
```

## No need to panic

Of course, aborting the program is not the only way to deal with errors.
Instead of the `panic!`, we can also easily write `return`:

```rust,no_run
# fn main() -> Result<(), Box<std::error::Error>> {
let result = std::fs::read_to_string("test.txt");
let _content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
# Ok(())
# }
```

This, however changes the return type our function needs.
Indeed, there was something hidden in our examples all this time:
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
It's the default return value of the function and means
"Result is okay, and has no content".

<aside>

**Aside:**
Why is this not written as `return Ok(());`?
It easily could be – this is totally valid as well.
The last expression of any block in Rust is its return value,
and it is customary to omit needless `return`s.

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

**Aside:**
There are a few more things happening here
that are not required to understand to work with this.
For example,
the error type in our `main` function is `Box<dyn std::error::Error>`.
But we've seen above that `read_to_string` returns a [`std::io::Error`].
This works because `?` expands to code that  _converts_ error types.

`Box<dyn std::error::Error>` is also an interesting type.
It's a `Box` that can contain _any_ type
that implements the standard [`Error`][`std::error::Error`] trait.
This means that basically all errors can be put into this box,
so we can use `?` on all of the usual functions that return `Result`s.

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## Providing Context

The errors you get when using `?` in your `main` function are okay,
but they are not great.
For example:
When you run `std::fs::read_to_string("test.txt")?`
but the file `test.txt` doesn't exist,
you get this output:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

In cases where your code doesn't literally contain the file name,
it would be very hard to tell which file was `NotFound`.
There are multiple ways to deal with this.

For example, we can create our own error type,
and then use that to build a custom error message:

```rust,ignore
{{#include errors-custom.rs}}
```

Now,
running this we'll get our custom error message:

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

Not very pretty,
but we can easily adapt the debug output for our type later on.

This pattern is in fact very common.
It has one problem, though:
We don't store the original error,
only its string representation.
The often used [`failure`] library has a neat solution for that:
Similar to our `CustomError` type,
it has a [`Context`] type
that contains a description as well as the original error.
The library also brings with it an extension trait ([`ResultExt`])
that adds [`context()`] and [`with_context()`] methods to `Result`.

[`failure`]: https://docs.rs/failure
[`Context`]: https://docs.rs/failure/0.1.3/failure/struct.Context.html
[`ResultExt`]: https://docs.rs/failure/0.1.3/failure/trait.ResultExt.html
[`context()`]: https://docs.rs/failure/0.1.3/failure/trait.ResultExt.html#tymethod.context
[`with_context()`]: https://docs.rs/failure/0.1.3/failure/trait.ResultExt.html#tymethod.with_context

To turn these wrapped error types
into something that humans will actually want to read,
we can further add the [`exitfailure`] crate,
and use its type as the return type of our `main` function.

Let's first import the crates by adding
`failure = "0.1.5"` and `exitfailure = "0.5.1"` to the `[dependencies]` section
of our `Cargo.toml` file.

The full example will then look like this:

[`exitfailure`]: https://docs.rs/exitfailure

```rust,ignore
{{#include errors-exit.rs}}
```

This will print an error:

```text
Error: could not read file `test.txt`
Info: caused by No such file or directory (os error 2)
```
