extern crate man;

use man::Man;

fn main() {
  let mut page = Man::new("basic");
  page.description("perform basic operations");
  page.author(&"Alice Person", Some(String::from("alice@person.com")));
  page.author("Bob Human", Some(String::from("bob@human.com")));
  // .flag(Some("-d"), Some("--debug"), Some("Activate debug mode"))
  // .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", page.render());
}
