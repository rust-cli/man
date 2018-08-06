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

use man::prelude::*;

fn main() {
  let page = Manual::new("basic")
    .about("A basic example")
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .flag(Flag::new().short("-d").long("--debug").description("Enable debug mode"))
    .flag(Flag::new().short("-v").long("--verbose").description("Enable verbose mode"))
    .option(Opt::new("output").short("-o").long("--output").description("The file path to write output to"));

  let _string = page.render();
}
```
Preview by running:
```sh
$ cargo run > /tmp/app.man; man /tmp/app.man
```
Which outputs:
```txt
BASIC(1)                                             General Commands Manual                                             BASIC(1)

NAME
       basic - A basic example

SYNOPSIS
       basic [FLAGS] [OPTIONS]

FLAGS
       -d, --debug
              Enable debug mode.

       -v, --verbose
              Enable verbose mode

OPTIONS
       -o, --output=output
              The file path to write output to.

EXIT STATUS
       0      Successful program execution.

       1      Unsuccessful program execution.

       101    The program panicked.

AUTHORS
         Alice Person <alice@person.com>
         Bob Human <bob@human.com>
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
