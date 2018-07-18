extern crate man;

use man::Man;

fn main() {
  let page = Man::new("basic")
    .description("perform basic operations")
    .author("Alice Person", Some("alice@person.com".into()))
    .author("Bob Human", Some("bob@human.com".into()))
    .flag(
      Some("-h".into()),
      Some("--help".into()),
      Some("Prints help information.".into()),
    )
    .flag(
      Some("-v".into()),
      Some("--verbose".into()),
      Some("Prints version information.".into()),
    );
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", page.render());
}
