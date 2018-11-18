# Packaging and rendering documentation

If you feel confident that your program is ready to for other people to use,
it is time to package and release it!

There are a few approaches,
and we'll look at three of them
from "quickest to set up" to "most convenient for users".

## Quickest: `cargo publish`

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

### How to install a binary from crates.io

Now that we've seen how to publish a crate to crates.io,
you might be wondering how to install it.
In contrast to libraries,
that cargo will download and compile for you
when you run `cargo build` (or a similar command),
you'll need to tell it to explicitly install binaries.

This is done using
`cargo install <crate-name>`.
It will by default download the crate,
compile all the binary targets it contains
(in "release" mode, so it might take a while)
and copy them into the `~/.cargo/bin/` directory.
(Make sure that your shell knows to looks there for binaries!)

It's also possible to
install crates from git repositories,
only install specific binaries of a crate,
and specify and alternative directory to install them to.
Best have a look at `cargo install --help`.

### When to use it

`cargo install` is a simple way to publish a binary crate.
It's very convenient for Rust developers to use,
but has some significant downsides:
Since it will always compile your source from scratch,
users of your tool will need to have
Rust, cargo, and all other system dependencies your project requires
installed on their machine.
Compiling large Rust code bases can also take some time.
Furthermore, there is no simple way to update tools installed with cargo:
User will need to run `cargo install` again at some point,
and pass the `--force` flag to overwrite the old binaries.

Best use this for distributing tools
that are targeted at other Rust developers.
For example:
A lot of cargo subcommands
like `cargo-tree` or `cargo-outdated`
can be installed with it.

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
