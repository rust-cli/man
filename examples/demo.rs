extern crate man;

use man::Man;

fn main() {
  let mut page = Man::new("basic");
  page.description("A basic example");
  page.author(&"Alice", Some(String::from("alice@email.com")));
  // .author("Bob", Some("bob@email.com"))
  // .flag(Some("-d"), Some("--debug"), Some("Activate debug mode"))
  // .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", page.render());
}
