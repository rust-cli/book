# Rendering documentation for your Configuration

If your application offers extensive configuration options, you may want to
render these for the conveniance of the user.

You could render them straight to the terminal, so that a user can "ask" your
app what configuration it provides:

```commandline
myapp print-config-options
```

Using the [type_description crate](https://crates.io/crates/type_description)
you can annotate your configuration struct(s) and let Rust do the magic:

```rust,ignore
use type_description::AsTypeDescription;
use type_description::TypeDescription;
use type_description::TypeKind;
use type_description::Sign;

/// A configuration
#[derive(TypeDescription)]
struct Config {
    /// The bind address
    addr: std::net::SocketAddr,

    /// The Port
    port: u16,
}
```

The derive macro implements the `AsTypeDescription` on your `Config` type and
uses the comments as descriptions.

The `type_description` crate also implements the `AsTypeDescription` trait for
the most common types for configuration from the standard library, such as
numbers, `bool`, `String`, but also `Option`, `Vec` or `HashMap`.

You can then use `Config::as_type_description()` to get the description for your
`Config` type.

You can render this information to a web-page, pdf, or simply to text and
print it on the commandline (render functionality
[within the type_description crate is currently unreleased](https://github.com/TheNeikos/type_description/blob/master/src/render.rs#L70)).

