extern crate man;

use man::Man;

fn main() {
  let page = Man::new("basic")
    .description("perform basic operations")
    .author(&"Alice Person", Some(String::from("alice@person.com")))
    .author("Bob Human", Some(String::from("bob@human.com")))
    .flag(
      Some("-h".into()),
      Some("--help".into()),
      Some("Print help information.".into()),
    )
    .flag(
      Some("-v".into()),
      Some("--verbose".into()),
      Some("Prints version information.".into()),
    );
  // .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", page.render());
}
