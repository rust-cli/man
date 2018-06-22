# man
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate structured man pages using
[roff-rs](https://github.com/killercup/roff-rs).

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate man;

use man::Man;

fn main() {
  let page = Man::new("basic")
    .description("A basic example")
    .author("Alice", Some("alice@email.com"))
    .author("Bob", Some("bob@email.com"))
    .flag(Some("-d"), Some("--debug"), Some("Activate debug mode"))
    .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
    .option(Some("-o"), Some("--output"), "output", None, "Output file");
}
```

## Installation
```sh
$ cargo add man
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/man.svg?style=flat-square
[2]: https://crates.io/crates/man
[3]: https://img.shields.io/travis/rust-clique/man.svg?style=flat-square
[4]: https://travis-ci.org/rust-clique/man
[5]: https://img.shields.io/crates/d/man.svg?style=flat-square
[6]: https://crates.io/crates/man
[7]: https://docs.rs/man/badge.svg
[8]: https://docs.rs/man
