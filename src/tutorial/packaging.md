# Packaging and rendering documentation

If you feel confident that your program is ready to for other people to use,
it is time to package and release it!

## The quick way: With cargo

The easiest way to publish your app is with cargo.
Do you remember how we added external dependencies to our project?
Cargo downloaded them from its default "crate registry", [crates.io].
With `cargo publish`,
you too can publish crates to [crates.io].
And this works for all crates,
including those with binary targets.

Publishing a crate to [crates.io] is pretty straight-forward:
If you haven't already, create an account on [crates.io].
Currently, this is done via authorizing you on Github,
so you'll need to have Github account
(and be logged in there).
Next, you need to log in with cargo.
For that, go to your
[crates.io account page],
create a new token,
and then run `cargo login <your-new-token>`.
You only need to do this once per computer.
You can learn more about this
in cargo's [publishing guide].

Now that cargo as well crates.io know you,
you can publish crates.
Before you hastily go ahead and publish a new crate version,
it's a good idea to open your `Cargo.toml` once more
and make sure you added a the necessary metadata.
You can find all the possible fields you can set
in the documentation for [cargo's manifest format].
Here's a quick overview of some common entries:

```toml
[package]
name = "grrs"
version = "0.1.0"
authors = ["Your Name <your@email.com>"]
license = "MIT OR Apache-2.0"
description = "A tool to search files"
readme = "README.md"
homepage = "https://github.com/you/grrs"
repository = "https://github.com/you/grrs"
keywords = ["cli", "search", "demo"]
categories = ["command-line-utilities"]
```

<aside class="note">

**Note:**
This example includes the mandatory license field
with a common choice for Rust projects:
The same license that is also used for the compiler itself.
It also refers to a `README.md` file.
It should include a quick description of what your project is about,
and will be included not only on the crates.io page of your crate,
but also what Github shows by default on repository pages.

</aside>

[crates.io]: https://crates.io/
[crates.io account page]: https://crates.io/me
[publishing guide]: https://doc.rust-lang.org/1.30.0/cargo/reference/publishing.html
[cargo's manifest format]: https://doc.rust-lang.org/1.30.0/cargo/reference/manifest.html

## Distributing binaries

## Getting your app into package repositories

<aside class="todo">

**TODO:** Talk about packaging on CI
[Issue #69](https://github.com/rust-lang-nursery/cli-wg/issues/69)

</aside>
<aside class="todo">

**TODO:** Talk about automatically generating Man pages in a build script
[Issue #70](https://github.com/rust-lang-nursery/cli-wg/issues/70)

</aside>
