# Output

## Printing "Hello World"

```rust
println!("Hello World");
```

Well, that was easy.
Great, onto the next topic.

## Using println

You can pretty much print all the things you like
with the `println!` macro.
This macro has some pretty amazing capabilities,
but also a special syntax.
It expects you to write a string literal as the first parameter,
that contains placeholders that will be filled in
by the values of the parameters that follow as further arguments.

For example:

```rust
let x = 42;
println!("My lucky number is {}.", x);
```

will print

```console
My lucky number is 42.
```

The curly braces (`{}`) in the string above is one of these placeholders.
This is the default placeholder type
that tries to print the given in a human readable way.
For numbers and strings this works very well,
but not all types can do that.
This is why there is also a "debug representation",
that you can get by filling the braces of the placeholder like this: `{:?}`.

For example,

```rust
let xs = vec![1, 2, 3];
println!("The list is: {:?}", xs);
```

will print

```console
The list is: [1, 2, 3]
```

If you want your own data types to be printable for debugging and logging,
you can in most cases add a `#[derive(Debug)]` above their definition.

<aside>

**Aside:**
The human readable printing is done using the [`Display`] trait,
debug output uses the [`Debug`] trait.
You can find more information about the syntax you can use in `println!`
in the [documentation for the `std::fmt` module][std::fmt].

[`Display`]: https://doc.rust-lang.org/1.27.2/std/fmt/trait.Display.html
[`Debug`]: https://doc.rust-lang.org/1.27.2/std/fmt/trait.Debug.html
[std::fmt]: https://doc.rust-lang.org/1.27.2/std/fmt/index.html

</aside>

## Printing errors

Printing errors should be done via `stderr`
to make it easier for users
and other tools
to pipe their outputs to files
or more tools.

In Rust this is achieved
with `println!` and `eprintln!`,
the former printing to `stdout`
and the latter to `stderr`.

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

<aside>

**Beware**: Printing escape codes can be dangerous,
putting the user's terminal into a weird state.
Always be careful when manually printing them!

Ideally you should be using a crate like `ansi_term`
when dealing with raw escape codes
to make your (and your user's) life easier.

</aside>

## Showing a progress bar

Some CLI applications run less than a second,
others take minutes or hours.
If you are writing one of the latter types of programs,
you might want to show the user that something is happening.
For this, you should try to printing useful status updates,
ideally in a form that can be easily consumed.

Using the [indicatif] crate,
you can add progress bars
and little spinners to your program.

<aside class="todo">

**TODO:**
Show an example like
[this](https://github.com/mitsuhiko/indicatif/blob/950091d1b1683a88e01c4d4975f591009f56322b/examples/log.rs)
or [this](https://github.com/ashleygwilliams/cargo-generate/blob/c18cba0b33764012e25288d43c6a8545222b96f4/src/main.rs#L95).
[Issue #67](https://github.com/rust-lang-nursery/cli-wg/issues/67)

</aside>

[indicatif]: https://crates.io/crates/indicatif

## Logging

To make it easier to understand what is happening in our program,
we might want to add some log statements.
This is usually easy while writing your application.
But it will become super helpful when running this program again in half a year.

<aside class="todo">

**TODO:**
`log` crate: macros with similar syntax to `println`
[Issue #68](https://github.com/rust-lang-nursery/cli-wg/issues/68)

</aside>

<aside class="todo">

**TODO:**
crate for actual log output – which one?
env_logger?
Link to `../in-depth/human-communication.html`
[Issue #68](https://github.com/rust-lang-nursery/cli-wg/issues/68)

</aside>

<aside>

**Aside:**
Experience has shown that even mildly useful CLI programs can end up being used for years to come.
(Especially if they were meant as a temporary solution.)
If your application doesn't work
and someone (e.g., you, in the future) needs to figure out why,
being able to pass `--verbose` to get additional log output
can make the difference between minutes and hours of debugging.

</aside>
