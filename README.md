# Versionator: version information for build script

[![Crates.io](https://img.shields.io/crates/v/versionator.svg?maxAge=2592000)](https://crates.io/crates/versionator)

[Documentation](https://docs.rs/crate/versionator)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
versionator = "1.0"
```

Next, add this to your build script (`build.rs`):

```rust
extern crate versionator;

fn main() {
    let version = versionisator::Version::new(
        env!("CARGO_MANIFEST_DIR"),
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string()
    );

    println!("cargo:rustc-env=FULL_VERSION={}", version.full());
    println!("cargo:rustc-env=SIMPLE_VERSION={}", version.simple());
    println!("cargo:rustc-env=SOURCE_VERSION={}", version.hash());
}
```

`FULL_VERSION` will give you:

```
versionator 1.0.0 (master-3326b9b+, debug, mac [x86_64]) - [rustc 1.37.0 (eae3437df 2019-08-13)]
\_________/ \___/  \____/ \_____/|  \___/  \_/  \____/      \_________________________________/
 |           |      |      |     |   |      |      |           |
 |           |      |      |     |   |      |      |           +- rustc --version
 |           |      |      |     |   |      |      +- std::env::consts::ARCH
 |           |      |      |     |   |      +- std::env::consts::OS
 |           |      |      |     |   +- Checks debug_assertions
 |           |      |      |     +- Adds a "+" if the working tree is not clean
 |           |      |      +- Commit hash
 |           |      +- Current branch name
 |           +- Package version from Cargo.toml
 +- Package name from Cargo.toml
```

## Credits

This package is inspired from [input-output-hk/jormungandr/719](https://github.com/input-output-hk/jormungandr/issues/719)
itself inspired from https://vallentin.io/2019/06/06/versioning which served as guide for the idea and the code.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bawawa` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.