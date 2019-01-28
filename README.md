# man
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate structured man pages using
[roff-rs](https://github.com/killercup/roff-rs).

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
use man::prelude::*;

fn main() {
    let page = Manual::new("basic")
        .about("A basic example")
        .author(Author::new("Alice Person").email("alice@person.com"))
        .author(Author::new("Bob Human").email("bob@human.com"))
        .flag(
            Flag::new()
                .short("-d")
                .long("--debug")
                .help("Enable debug mode"),
        )
        .flag(
            Flag::new()
                .short("-v")
                .long("--verbose")
                .help("Enable verbose mode"),
        )
        .option(
            Opt::new("output")
                .short("-o")
                .long("--output")
                .help("The file path to write output to"),
        )
        .example(
            Example::new()
                .text("run basic in debug mode")
                .command("basic -d")
                .output("Debug Mode: basic will print errors to the console")
            )
        .custom(
            Section::new("usage note")
                .paragraph("This program will overwrite any file currently stored at the output path")
        )
        .exit_status(ExitStatus::default())
        .render();

    println!("{}", page);
}
```
Preview by running:
```sh
$ cargo run > /tmp/app.man; man /tmp/app.man
```
Which outputs:
<pre>
BASIC(1)                       General Commands Manual                          BASIC(1)

<b>NAME</b>
       basic - A basic example

<b>SYNOPSIS</b>
       <b>basic</b> [FLAGS] [OPTIONS]

<b>FLAGS</b>
       <b>-d, --debug</b>
              Enable debug mode

       <b>-v, --verbose</b>
              Enable verbose mode

<b>OPTIONS</b>
       <b>-o, --output</b>=<i>output</i>
              The file path to write output to

<b>USAGE NOTE</b>
       This file will overwrite any file currently stored at the output path.

<b>EXIT STATUS</b>
       <strong>0</strong>      Successful program execution.

       <b>1</b>      Unsuccessful program execution.

       <b>101</b>    The program panicked.

<b>EXAMPLES</b>
       run basic in debug mode
              <b>$ basic -d</b>
              Debug Mode: basic will print errors to the console

<b>AUTHORS</b>
         Alice Person &lt;alice@person.com&gt;
         Bob Human &lt;bob@human.com&gt;

                                                                                BASIC(1)
</pre>

## Installation
If using [cargo-edit](https://github.com/killercup/cargo-edit), install with
```sh
$ cargo add man
```
Otherwise, install by adding to Cargo.toml file's dependency section.

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
