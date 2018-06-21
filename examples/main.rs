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
