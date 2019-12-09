# Signal handling

Processes
like command line applications
need to react to signals sent by the operating system.
The most common example is probably <kbd>Ctrl</kbd>+<kbd>C</kbd>,
the signal that typically tells a process to terminate.
To handle signals in Rust programs
you need to consider how you can receive these signals
as well as how you can react to them.

<aside>

**Note:**
If your applications does not need to gracefully shutdown,
the default handling is fine
(i.e. exit immediately
and let the OS cleanup resources like open file handles).
In that case:
No need to do what this chapter tells you!

However,
for applications that need to clean up after themselves,
this chapter is very relevant!
For example,
if your application needs to
properly close network connections
(saying "good bye" to the processes at the other end),
remove temporary files,
or reset system settings,
read on.

</aside>

## Differences between operating systems

On Unix systems
(like Linux, macOS, and FreeBSD)
a process can receive [signals].
It can either react to them
in a default (OS-provided) way,
catch the signal and handle them in a program-defined way,
or ignore the signal entirely.

[signals]: https://manpages.ubuntu.com/manpages/bionic/en/man7/signal.7.html

Windows does not have signals.
You can use [Console Handlers]
to define callbacks that get executed when an event occurs.
There is also [structured exception handling]
which handles all the various types of system exceptions such as division by zero, invalid access exceptions, stack overflow, and so on

[Console Handlers]: https://docs.microsoft.com/de-de/windows/console/console-control-handlers
[structured exception handling]: https://docs.microsoft.com/en-us/windows/desktop/debug/structured-exception-handling

## First off: Handling Ctrl+C

The [ctrlc] crate does just what the name suggests:
It allows you to react to the user pressing <kbd>Ctrl</kbd>+<kbd>C</kbd>,
in a cross-platform way.
The main way to use the crate is this:

[ctrlc]: https://crates.io/crates/ctrlc

```rust,ignore
{{#include signals-ctrlc.rs}}
```

This is, of course, not that helpful:
It only prints a message but otherwise doesn't stop the program.

In a real-world program,
it's a good idea to instead set a variable in the signal handler
that you then check in various places in your program.
For example,
you can set an `Arc<AtomicBool>`
(a boolean shareable between threads)
in your signal handler,
and in hot loops,
or when waiting for a thread,
you periodically check its value
and break when it becomes true.

## Handling other types of signals

The [ctrlc] crate only handles <kbd>Ctrl</kbd>+<kbd>C</kbd>,
or, what on Unix systems would be called `SIGINT` (the "interrupt" signal).
To react to more Unix signals,
you should have a look at [signal-hook].
Its design is described in [this blog post][signal-hook-post],
and it is currently the library with the widest community support.

Here's a simple example:

```rust,ignore
{{#include signals-hooked.rs}}
```

[signal-hook-post]: https://vorner.github.io/2018/06/28/signal-hook.html

## Using channels

Instead of setting a variable
and having other parts of the program check it,
you can use channels:
You create a channel into which the signal handler emits a value
whenever the signal is received.
In your application code you use
this and other channels
as synchronization points between threads.
Using [crossbeam-channel] it would look something like this:

[crossbeam-channel]: https://crates.io/crates/crossbeam-channel

```rust,ignore
{{#include signals-channels.rs}}
```

## Using futures and streams

If you are using [tokio],
you are most likely already writing your application
with asynchronous patterns and an event-driven design.
Instead of using crossbeam's channels directly,
you can enable signal-hook's `tokio-support` feature.
This allows you to call [`.into_async()`]
on signal-hook's `Signals` types
to get a new type that implements `futures::Stream`.

[signal-hook]: https://crates.io/crates/signal-hook
[tokio]: https://tokio.rs/
[`.into_async()`]: https://docs.rs/signal-hook/0.1.6/signal_hook/iterator/struct.Signals.html#method.into_async

## What to do when you receive another Ctrl+C while you're handling the first Ctrl+C

Most users will press <kbd>Ctrl</kbd>+<kbd>C</kbd>,
and then give your program a few seconds to exit,
or tell them what's going on.
If that doesn't happen,
they will press <kbd>Ctrl</kbd>+<kbd>C</kbd> again.
The typical behavior is to have the application quit immediately.
