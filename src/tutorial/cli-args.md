## Parsing command line arguments

A typical invocation of our CLI tool will look like this:
`grrs foobar test.txt`.
You can think of CLI arguments as a data type.
In our case, we have two fields,
`pattern` (the string to look for),
and `path` (the file to look in).
In Rust, it is very common to try and structure programs around the data they deal with.
This is a good start:

```rust
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```

<aside>

**Aside:** `PathBuf` is like a `String` but for file system paths that works cross-platform.

</aside>

Now, we still need to actually get the arguments the user passed into this form.
One option would be manually parse the list of strings we get from the operating system,
but a much nicer way is to use one of the many available libraries.
As you can see in the `src/main.rs` file,
our templates already contains some code using `clap`,
and in particular use it’s “derive” feature.
This is quite nice:
All we have to do is annotate a struct and it’ll generate the code that parses the arguments into the fields.
Let’s add our fields to the `Cli` struct in the template 
and also some documentation comments along the way.
It’ll look like this:

```rust
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Clap)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}
```

Right below the `Cli` struct our template contains its `main` function.
When the program starts, it will call this function.
The first line is

```rust
let args = Cli::parse();
```

This will try to parse the arguments the user gave when executing the program into our `Cli` struct.
You might be wondering what happens if this fails.
Give it a try!
