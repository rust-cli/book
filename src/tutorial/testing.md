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

## Making you code testable

There are two complementary approaches to testing functionality:
Testing the small units that you build your complete application from,
and testing the final application "from the outside".
Let's begin with the first one.

To figure our what we should test,
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
and we'll make use of one called `std::fmt::Write`.
This is a trait that abstract over things we can write to,
which includes strings but also `stdout`.

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

Now we can test for the ouptut:

```rust
#[test]
fn find_a_match() {
    let mut result = String::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, "lorem ipsum\n");
}
```

<aside class="exercise">

**Exercise for the reader:**
`writeln!` returns a `Result`. Add error handling to `find_matches`.

</aside>

## Splitting your code into library and binary targets

<aside class="todo">

**TODO:**
Move "testable"/pure functions into "library" part,
write unit tests/docs tests.
Only makes sense if there are enough.
Otherwise keep in main.rs.
Discuss advantages of thinking of library API.
[Issue #71](https://github.com/rust-lang-nursery/cli-wg/issues/71)

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
