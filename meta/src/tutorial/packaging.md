# Packaging and distributing a Rust tool

If you feel confident that your program is ready for other people to use,
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

Publishing a crate to [crates.io] is pretty straightforward:
If you haven't already, create an account on [crates.io].
Currently, this is done via authorizing you on GitHub,
so you'll need to have a GitHub account
(and be logged in there).
Next, you log in using cargo on your local machine.
For that, go to your
[crates.io account page],
create a new token,
and then run `cargo login <your-new-token>`.
You only need to do this once per computer.
You can learn more about this
in cargo's [publishing guide].

Now that cargo as well as crates.io know you,
you are ready to publish crates.
Before you hastily go ahead and publish a new crate (version),
it's a good idea to open your `Cargo.toml` once more
and make sure you added the necessary metadata.
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
but also what GitHub shows by default on repository pages.

</aside>

[crates.io]: https://crates.io/
[crates.io account page]: https://crates.io/me
[publishing guide]: https://doc.rust-lang.org/1.39.0/cargo/reference/publishing.html
[cargo's manifest format]: https://doc.rust-lang.org/1.39.0/cargo/reference/manifest.html

### How to install a binary from crates.io

We've seen how to publish a crate to crates.io,
and you might be wondering how to install it.
In contrast to libraries,
which cargo will download and compile for you
when you run `cargo build` (or a similar command),
you'll need to tell it to explicitly install binaries.

This is done using
`cargo install <crate-name>`.
It will by default download the crate,
compile all the binary targets it contains
(in "release" mode, so it might take a while)
and copy them into the `~/.cargo/bin/` directory.
(Make sure that your shell knows to look there for binaries!)

It's also possible to
install crates from git repositories,
only install specific binaries of a crate,
and specify an alternative directory to install them to.
Have a look at `cargo install --help` for details.

### When to use it

`cargo install` is a simple way to publish a binary crate.
It's very convenient for Rust developers to use,
but has some significant downsides:
Since it will always compile your source from scratch,
users of your tool will need to have
Rust, cargo, and all other system dependencies your project requires
to be installed on their machine.
Compiling large Rust codebases can also take some time.

Furthermore, there is no simple way to update tools installed with cargo:
The user will need to run `cargo install` again at some point,
and pass the `--force` flag to overwrite the old binaries.
This is a [missing feature][cargo-issue-2082]
and there are subcommands [like this one][cargo-update]
you can install to add that,
though.

[cargo-issue-2082]: https://github.com/rust-lang/cargo/issues/2082
[cargo-update]: https://crates.io/crates/cargo-update

It's best to use this for distributing tools
that are targeted at other Rust developers.
For example:
A lot of cargo subcommands
like `cargo-tree` or `cargo-outdated`
can be installed with it.

## Distributing binaries

Rust is a language that compiles to native code
and by default statically links all dependencies.
When you run `cargo build`
on your project that contains a binary called `grrs`,
you'll end up with a binary file called `grrs`.
Try it out:
Using `cargo build`, it'll be `target/debug/grrs`,
and when you run `cargo build --release`, it'll be `target/release/grrs`.
Unless you use crates
that explicitly need external libraries to be installed on the target system
(like using the system's version of OpenSSL),
this binary will only depend on common system libraries.
That means,
you take that one file, 
send it to people running the same operating system as you,
and they'll be able to run it.

This is already very powerful!
It works around two of the downsides we just saw for `cargo install`:
There is no need to have Rust installed on the user's machine,
and instead of it taking a minute to compile,
they can instantly run the binary.

So, as we've seen,
`cargo build` _already_ builds binaries for us.
The only issue is,
those are not guaranteed to work on all platforms.
If you run `cargo build` on your Windows machine,
you won't get a binary that works on a Mac by default.
Is there a way to generate these binaries
for all the interesting platforms
automatically?

### Building binary releases on CI

If your tool is open sourced
and hosted on GitHub,
it's quite easy to set up a free CI (continuous integration) service
like [Travis CI].
(There are other services that also work on other platforms, but Travis is very popular.)
This basically runs setup commands
in a virtual machine
each time you push changes to your repository.
What those commands are,
and the types of machines they run on,
is configurable.
For example:
A good idea is to run `cargo test`
on a machine with Rust and some common build tools installed.
If this fails,
you know there are issues in the most recent changes.

[Travis CI]: https://travis-ci.com/

We can also use this
to build binaries and upload them to GitHub!
Indeed, if we run
`cargo build --release`
and upload the binary somewhere,
we should be all set, right?
Not quite.
We still need to make sure the binaries we build
are compatible with as many systems as possible.
For example,
on Linux we can compile not for the current system,
but instead for the `x86_64-unknown-linux-musl` target,
to not depend on default system libraries.
On macOS, we can set `MACOSX_DEPLOYMENT_TARGET` to `10.7`
to only depend on system features present in versions 10.7 and older.

You can see one example of building binaries using this approach
[here][wasm-pack-travis] for Linux and macOS
and [here][wasm-pack-appveyor] for Windows (using AppVeyor).

[wasm-pack-travis]: https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.travis.yml#L74-L91
[wasm-pack-appveyor]: https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.appveyor.yml

Another way is to use pre-built (Docker) images
that contain all the tools we need
to build binaries.
This allows us to easily target more exotic platforms, too.
The [trust] project contains
scripts that you can include in your project
as well as instructions on how to set this up.
It also includes support for Windows using AppVeyor.

If you'd rather set this up locally
and generate the release files on your on machine,
still have a look at trust.
It uses [cross] internally,
which works similar to cargo
but forwards commands to a cargo process inside a Docker container.
The definitions of the images are also available in
[cross' repository][cross].

[trust]: https://github.com/japaric/trust
[cross]: https://github.com/rust-embedded/cross

### How to install these binaries

You point your users to your release page
that might look something [like this one][wasm-pack-release],
and they can download the artifacts we've just created.
The release artifacts we've just generated are nothing special:
At the end, they are just archive files that contain our binaries!
This means that users of your tool
can download them with their browser,
extract them (often happens automatically),
and copy the binaries to a place they like.

[wasm-pack-release]: https://github.com/rustwasm/wasm-pack/releases/tag/v0.5.1

This does require some experience with manually "installing" programs,
so you want to add a section to your README file
on how to install this program.

<aside class="note">

**Note:**
If you used [trust] to build your binaries and added them to GitHub releases,
you can also tell people to run
`curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git your-name/repo-name`
if you think that makes it easier.

</aside>

### When to use it

Having binary releases is a good idea in general,
there's hardly any downside to it.
It does not solve the problem of users having to manually
install and update
your tools,
but they can quickly get the latest releases version
without the need to install Rust.

### What to package in addition to your binaries

Right now,
when a user downloads our release builds,
they will get a `.tar.gz` file
that only contains binary files.
So, in our example project,
they will just get a single `grrs` file they can run.
But there are some more files we already have in our repository
that they might want to have.
The README file that tells them how to use this tool,
and the license file(s),
for example.
Since we already have them,
they are easy to add.

There are some more interesting files
that make sense especially for command-line tools,
though:
How about we also ship a man page in addition to that README file,
and config files that add completions of the possible flags to your shell?
You can write these by hand,
but _clap_, the argument parsing library we use
(which structopt builds upon)
has a way to generate all these files for us.
See [this in-depth chapter][clap-man-pages]
for more details.


[clap-man-pages]: ../in-depth/docs.html


## Getting your app into package repositories

Both approaches we've seen so far
are not how you typically install software on your machine.
Especially command-line tools
you install using global package managers
on most operating systems.
The advantages for users are quite obvious:
There is no need to think about how to install your program,
if it can be installed the same way as they install the other tools.
These package managers also allow users to update their programs
when a new version is available.

Sadly, supporting different systems means
you'll have to look at how these different systems work.
For some,
it might be as easy as adding a file to your repository
(e.g. adding a Formula file like [this][rg-formula] for macOS's `brew`),
but for others you'll often need to send in patches yourself
and add your tool to their repositories.
There are helpful tools like
[cargo-rpm](https://crates.io/crates/cargo-rpm) and
[cargo-deb](https://crates.io/crates/cargo-deb),
but describing how they work
and how to correctly package your tool
for those different systems is beyond the scope of this chapter.

[rg-formula]: https://github.com/BurntSushi/ripgrep/blob/31adff6f3c4bfefc9e77df40871f2989443e6827/pkg/brew/ripgrep-bin.rb

Instead,
let's have a look at a tool that is written in Rust
and that is available in many different package managers.

### An example: ripgrep

[ripgrep] is an alternative to `grep`/`ack`/`ag` and is written in Rust.
It's quite successful and is packaged for many operating systems:
Just look at [the "Installation" section][rg-install] of its README!

Note that it lists a few different options how you can install it:
It starts with a link to the GitHub releases
which contain the binaries so you can download them directly;
then it lists how to install it using a bunch of different package managers;
finally, you can also install it using `cargo install`.

This seems like a very good idea:
Don't pick and choose one of the approaches presented here,
but start with `cargo install`,
add binary releases,
and finally start distributing your tool using system package managers.

[ripgrep]: https://github.com/BurntSushi/ripgrep
[rg-install]: https://github.com/BurntSushi/ripgrep/tree/31adff6f3c4bfefc9e77df40871f2989443e6827#installation
