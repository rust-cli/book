# Useful crates
There is always new crates being released that can be useful in the development of command line applications.

## Crates referenced in this book
- [anyhow](https://crates.io/crates/anyhow) - provides `anyhow::Error` for easy error handling
- [asset_cmd](https://crates.io/crates/assert_cmd) - simplifies integration testing of CLIs
- [atty](https://crates.io/crates/atty) - detected whether application is running in a tty
- [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag) - adds a `--verbose` flag to structopt CLIs
- [clap](https://crates.io/crates/clap) - command line argument parser
- [confy](https://crates.io/crates/confy) - boilerplate-free configuration management
- [convey](https://crates.io/crates/convey) - easy output for machines and humans
- [crossbeam-channel](https://crates.io/crates/crossbeam-channel) - provides multi-producer multi-consumer channels for message passing
- [ctrlc](https://crates.io/crates/ctrlc) - easy ctrl-c handler
- [env_logger](https://crates.io/crates/env_logger) - implements a logger configurable via environment variables
- [exitcode](https://crates.io/crates/exitcode) - system exit code constants
- [human-panic](https://crates.io/crates/human-panic) - panic message handler
- [indicatif](https://crates.io/crates/indicatif) - progress bars and spinners
- [log](https://crates.io/crates/log) - provides logging abstracted over implementation
- [predicates](https://crates.io/crates/predicates) - implements boolean-valued predicate functions
- [proptest](https://crates.io/crates/proptest) - property testing framework
- [serde_json](https://crates.io/crates/serde_json) - serialize/deserialize to JSON
- [signal-hook](https://crates.io/crates/signal-hook) - handles UNIX signals
- [structopt](https://crates.io/crates/structopt) - parses command line arguments into a struct
- [tokio](https://crates.io/crates/tokio) - asynchronous runtime
- [wasm-pack](https://crates.io/crates/wasm-pack) - tool for building WebAssembly

## Other crates
Due to the constantly-changing landscape of Rust crates, a good place to find crates is the [lib.rs](lib.rs) crate index.
Here are a few specific categories that might be useful for building CLI's:
- [Command-line interface](https://lib.rs/command-line-interface)
- [Configuration](https://lib.rs/config)
- [Database interfaces](https://lib.rs/database)
- [Encoding](https://lib.rs/encoding)
- [Filesystem](https://lib.rs/filesystem)
- [HTTP Client](https://lib.rs/web-programming/http-client)
- [Operating systems](https://lib.rs/os)
