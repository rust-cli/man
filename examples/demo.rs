extern crate man;

use man::Man;

fn main() {
  let page = Man::new("basic")
    .description("perform basic operations")
    .author(&"Alice Person", Some(String::from("alice@person.com")))
    .author("Bob Human", Some(String::from("bob@human.com")))
    .flag(
      Some("-d".into()),
      Some("--debug".into()),
      Some("activate debug mode".into()),
    );
  // .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", page.render());
}
