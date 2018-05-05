Existing packaging systems
- [cargo-deb](https://github.com/mmstick/cargo-deb)
- [cargo-wix\](https://github.com/volks73/cargo-wix)
- [cargo-arch](https://crates.io/crates/cargo-arch)
- [cargo-rpm](https://github.com/iqlusion-io/crates/tree/master/cargo-rpm)
- [cargo-ebuild](https://github.com/cardoe/cargo-ebuild)
- [cargo-tarball](https://github.com/crate-ci/cargo-tarball)

Related distribution methods
- [self_update](https://github.com/jaemk/self_update)
- [trust](https://github.com/japaric/trust/)

## Challenges in packaging

### Finding binary artifacts

Challenges
- Cross-compiling puts items under a different folder

Possible solutions
- Cargo gained support for an `--out-dir`, see [rust-lang/cargo#5203](https://github.com/rust-lang/cargo/pull/5203)
  - When available in stable?

### Finding `build.rs` artifacts

Examples
- Completions
- man pages

Challenges
- `build.rs` scripts generally write to
  [`OUT_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
  which have non-predictable paths like
  `target/debug/build/cargo-tarball-b8d713fa01725c21/out/`.

Existing discussions
- [How to get OUT_DIR without running the build script?](https://users.rust-lang.org/t/how-to-get-out-dir-without-running-the-build-script/17239/3)
- [Location of generated non-binary artifacts](https://internals.rust-lang.org/t/location-of-generated-non-binary-artifacts/7430)

Workarounds:
- Don't use `build.rs` but generate an extra `bin` target to build these as part of the CI

Possible solutions
- [rust-lang/cargo#5457](https://github.com/rust-lang/cargo/issues/5457)

### Misc

- Knowing the `bin` name
  - On Windows, it will have a `.exe` extension
  - Could use https://doc.rust-lang.org/std/env/consts/index.html
  - Can't if you want cross-compilation
- Signing
  - debian: on the repo
  - windows: embedded in the exe
  - mac: in a paralel file or extended attributes
    - Using extended attributes can be lost when tarballing for transfer
- Knowing the multiarch triplet for Ubuntu
  - See [`cargo-deb`'s workaround](https://github.com/mmstick/cargo-deb/blob/master/src/manifest.rs#L697)
  - ARM is where this can get weird
  - `std::env::ARCH` is ambiguous
- Identifying the target triplet
  - Not returned by `cargo-metadata`
  - Not available as an env variable when compiling the program
  - Workaround: [use a build script](https://github.com/mmstick/cargo-deb/blob/master/build.rs)
  - Note: that is used for identifying the default target triplet, can be overridden on the command line
- Desire for a crate generating multiple packages
  - Linux packages: arch vs noarch
  - `-dbg` packages, see https://github.com/mmstick/cargo-deb/issues/62
  - A workspace might want to be a single package or a package per type of artifact

## Stager

Grand vision: All packaging tools use the same configuration schema; data and
fields are just specialized to the individual distribution needs

Ways to accomplish this
- Tiered configuration
  - `cargo-deb` would read from its own section of configuration, falling back to a general config for unspecified fields
  - `Cargo.toml` already provides a lot of what the general config could be
- Common, consistent patterns for
  - workspaces
  - defining multiple top-level packages (ignoring `-dbg` and so on)
- Common format for describing layout of files along with metadata, like permissions, and where the files can be sourced from
  - Introducing [`stager`](https://github.com/crate-ci/stager)


Or put another way, a Rust ecosystem equivalent of [sbt native packager](https://sbt-native-packager.readthedocs.io/en/stable/)

Example client: [`cargo-tarball`](https://github.com/crate-ci/cargo-tarball) which is meant to replace `trust` boilerplate.

Status
- stager: ready for integration into packaging tools to identify missing features
- cargo-tarball: Almost self-releasing, just missing a few features

### What is stager

Stager defines a common format for describing how to layout files in a package
and provides the tools to make this happen.

At the moment, the format leverages the [liquid template
language](https://shopify.github.io/liquid/) for making life easier for stager
dependents on their users.

Currently, all of the data is staged into a directory.  This could evolve
to support in-memory staging where files are directly written to compressed files.

### Open Issues

#### Packaging configuration home

Should we put this packaging configuration inside a `Cargo.toml` `[metadata]` table or in a separate file?

#### Auto-generate binary artifacts

Create cargo source for stager to run cargo and generate the binary to then stage.

#### Compression

For `deb`, documentation should be compressed.
- Created [crate-ci/stager#18](https://github.com/crate-ci/stager/issues/18) in response
- cargo-deb wants to abstract these rules
  - This policy can't live in stager
  - Will need to walk the staging configuration, identifying documentation, and mutate them to enable compression.
