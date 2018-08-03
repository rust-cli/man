extern crate man;

use man::prelude::*;

fn main() {
  let msg = Man::new("auth-service")
    .description("authorize & authenticate members".into())
    .argument("path")
    .environment(Env::new("PORT").description("The network port to listen to"))
    .flag(
      Flag::new()
        .short("-h")
        .long("--help")
        .description("Prints help information"),
    )
    .flag(
      Flag::new()
        .short("-V")
        .long("--version")
        .description("Prints version information"),
    )
    .flag(
      Flag::new()
        .short("-v")
        .long("--verbosity")
        .description("Pass multiple times to print more information"),
    )
    .option(
      Opt::new("port")
        .short("-p")
        .long("--port")
        .description("The network port to listen to"),
    )
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .render();
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", msg);
}
