# CLI working group

This repo is for coordinating the work of the Rust CLI Working Group,
also known as "Rust CLIQuE" (Rust CLI Quality Enhancement).

It also contains the CLAiR, the [Command Line Applications in Rust][clair] book.

[clair]: https://rust-lang-nursery.github.io/cli-wg/

- [Working groups?](https://internals.rust-lang.org/t/announcing-the-2018-domain-working-groups/6737)
- [Announcement of this WG](https://internals.rust-lang.org/t/announcing-the-cli-working-group/6872/1)
- Chat with us
  - [Discord](https://discord.gg/dwq4Zme)

## Our goal

Let's make this a true statement:

Rust makes writing crossplatform, tested, modern command line applications frictionless
while incorporating industry best practices and providing great documentation.

## What's a CLI?

For our intents and purposes, a CLI is any program that

* Launches in a terminal
* Accepts configuration from various sources, such as command line arguments, environment variables, or configuration files 
* Runs to completion with minimal/no user interaction 
* Accepts input from `stdin`, files, or network
* Performs processing on some input (files, network, `stdin`) based on the configuration specified
* Communicates via standard outputs (files, network, `std{out,err}`) 

(We [specifically][i4] don't want to focus on "TUI" apps right now.)

[i4]: https://github.com/rust-lang-nursery/cli-wg/issues/4
