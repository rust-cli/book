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
I don't know about you,
but I really like to tell computers to do things for me.
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
All the magic is done by the `#[test]` attribute.
Just write a function that has some assertions in it
and attach that attribute to it.

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

## Splitting your code into library and binary targets

<aside class="todo">

**TODO:**
Move "testable"/pure functions into "library" part,
write unit tests/docs tests.
Only makes sense if there are enough.
Otherwise keep in main.rs.
Discuss advantages of thinking of library API.

</aside>

## Testing CLI applications by running them

<aside class="todo">

**TODO:** Talk about using assert_cli’s features to quickly run cargo binaries with different inputs and assert their outputs.

</aside>
<aside class="todo">

**TODO:** Talk about generating temp dirs with demo files.

</aside>
