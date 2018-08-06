extern crate man;

use man::prelude::*;

fn main() {
  let page = Manual::new("basic")
    .about("A basic example")
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .flag(Flag::new().short("-d").long("--debug").help("Enable debug mode"))
    .flag(Flag::new().short("-v").long("--verbose").help("Enable verbose mode"))
    .option(Opt::new("output").short("-o").long("--output").help("The file path to write output to"));

  let string = page.render();
  println!("{}", string);
}
