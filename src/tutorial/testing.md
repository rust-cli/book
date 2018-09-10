# Testing

Over the decades of people doing software development
one truth has been found:
Untested software rarely works.
(Many people would go so far and add
"Most tested software doesn't work either."
But we are all optimists here, right?)
So, to ensure that your program does what you expect it to do,
it is wise to test it.

One easy way to do that is
to write a `README` file
that describes what your program should do.
And when you feel ready to make a new release,
go through the `README` and ensure that
the behavior is still as expected.
You can make this a more rigorous exercise
by also writing down how your program should react to erroneous inputs.

Here's another fancy idea:
Write that `README` before you write the code.

<aside>

**Aside:**
Have a look at
[Test-driven development](https://en.wikipedia.org/wiki/Test-driven_development) (TDD)
if you haven't heard of it.

</aside>

## Automated testing

Now, this is all fine and dandy,
but doing all of this manually?
That can take a lot of time.
At the same time,
many people have come to enjoy telling computers to do things for them.
Let's talk about how to automate these tests.

Rust has a built-in test framework,
so let's start by writing a first test:

```rust
#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
```

You can put this snippet of code in pretty much any file
and `cargo test` will find
and run it.
The key here is the `#[test]` attribute.
It allows the build system to discover such functions
and run them as tests,
verifying that they don't panic.

<aside class="exercise">

**Exercise for the reader:**
Make this test work.

You should end up with output like the following:

```text
running 1 test
test check_answer_validity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</aside>

Now that we've seen *how* we can write tests,
we still need to figure out *what* to test.
As you've seen it's fairly easy to write assertions
for functions.
But a CLI application is often more than one function!
Worse, it often deals with user input,
reads files,
and writes output.

## Making your code testable

There are two complementary approaches to testing functionality:
Testing the small units that you build your complete application from,
these are called "unit tests".
There is also testing the final application "from the outside"
called "black box tests" or "integration tests".
Let's begin with the first one.

To figure out what we should test,
let's see what our program features are.
Mainly, `grrs` is supposed to print out the lines that match a given pattern.
So, let's write unit tests for _exactly this_:
We want to ensure that our most important piece of logic works,
and we want to do it in a way that is not dependent
on any of the setup code we have around it
(that deals with CLI arguments, for example).

Going back to our [first implementation](../impl-draft.md) of `grrs`,
we added this block of code to the `main` function:

```rust
// ...
for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
```

Sadly, this is not very easy to test.
First off all, it's in the main function, so we can't easily call it.
This is easily fixed by moving this piece of code into a function:

```rust
fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
```

Now we can call this function in our test,
and see what its output is:

```rust
#[test]
fn find_a_match() {
    find_matches("lorem ipsum\ndolor sit amet", "lorem");
    assert_eq!( // uhhhh
```

Or… can we?
Right now, `find_matches` prints directly to `stdout`, i.e., the terminal.
We can't easily capture this in a test!
This is a problem that often comes up
when writing tests after the implementation:
We have written a function that is firmly integrated
in the context it is used in.

<aside class="note">

**Note:**
This is totally fine when writing small CLI applications.
There's no need to make everything testable!
It is important to think about
which parts of your code you might want to write unit tests for, however.
While we'll see that it's easy to change this function to be testable,
this is not always the case.

</aside>

Alright, how can we make this testable?
We'll need to capture the output somehow.
Rust's standard library has some neat abstractions
for dealing with I/O (input/output)
and we'll make use of one called [`std::io::Write`].
This is a trait that abstract over things we can write to,
which includes strings but also `stdout`.

[`std::io::Write`]: https://doc.rust-lang.org/1.28.0/std/io/trait.Write.html

<aside class="note">

**Note:**
We could also make this function return a `String`,
but that would change the behavior.
Instead of writing to the terminal directly,
it would then collect everything into a string,
and dump all the results in one go at the end.

</aside>

Let's change our function to also accept a parameter `writer`
that implements `Write`.
In our test, we can then supply a simple string
to make assertions on.
Instead of `println!(…)` we can just use `writeln!(writer, …)`.

```rust
fn find_matches<W: Write>(content: &str, pattern: &str, writer: &mut W) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
```

Now we can test for the output:

```rust
#[test]
fn find_a_match() {
    let mut result = String::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, "lorem ipsum\n");
}
```

To now use this in our application code,
we have to change the call to `find_matches` in `main`
by adding [`&mut std::io::stdout()`][stdout] as the third parameter.

[stdout]: https://doc.rust-lang.org/1.28.0/std/io/fn.stdout.html

<aside class="exercise">

**Exercise for the reader:**
[`writeln!`] returns an [`io::Result`]. Add error handling to `find_matches`.

[`writeln!`]: https://doc.rust-lang.org/1.28.0/std/macro.writeln.html
[`io::Result`]: https://doc.rust-lang.org/1.28.0/std/io/type.Result.html

</aside>

We've just seen how to be make it easily testable,
we have

1. identified one of the core pieces of our application,
2. put it into its own function,
3. and made it more flexible.

Even though the goal was to make it testable,
the result we ended up with
is actually a very idiomatic and reusable piece of Rust code.
That's awesome!

## Splitting your code into library and binary targets

There's one more step we can go here.
So far we've put everything we wrote into the `src/main.rs` file.
This means our current project produces a single binary.
But we can also make our code available as a library, like this:

1. Put the `find_matches` function into a new `src/lib.rs`.
2. Add a `pub` in front of the `fn` (so it's `pub fn find_matches`)
   to make it something that users of our library can access.
3. Remove `find_matches` from `src/main.rs`,
   and instead add an `extern crate grrs;` on top.
4. In the `fn main`, prepend the call to `find_matches` with `grrs::`,
   so it's now `grrs::find_matches(…)`.
   This means it uses the function from the library we just wrote!

The way Rust deals with projects is quite flexible
and it's a good idea to think about
what to put into the library part of your crate early on.
You can for example think about writing a library
for your application-specific logic first
and then use it in your CLI just like any other library.
Or, if your project has multiple binaries,
you can put the common functionality into the library part of that crate.

<aside class="note">

**Note:**
Speaking of putting everything into a `src/main.rs`:
If we can continue to do that,
it'll become difficult to read.
[The module system](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)
can help you structure and organize your code.

</aside>


## Testing CLI applications by running them

<aside class="todo">

**TODO:** Talk about using assert_cmd’s features to quickly run cargo binaries with different inputs and assert their outputs.
[Issue #72](https://github.com/rust-lang-nursery/cli-wg/issues/72)

</aside>
<aside class="todo">

**TODO:** Talk about generating temp dirs with demo files.
[Issue #72](https://github.com/rust-lang-nursery/cli-wg/issues/72)

</aside>
