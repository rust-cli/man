extern crate man;

use man::prelude::*;

fn main() {
  let msg = Manual::new("auth-service")
    .about("authorize & authenticate members".into())
    .arg(Arg::new("path"))
    .env(Env::new("PORT").help("The network port to listen to"))
    .flag(
      Flag::new()
        .short("-h")
        .long("--help")
        .help("Prints help information."),
    )
    .flag(
      Flag::new()
        .short("-V")
        .long("--version")
        .help("Prints version information."),
    )
    .flag(
      Flag::new()
        .short("-v")
        .long("--verbosity")
        .help("Pass multiple times to print more information."),
    )
    .option(
      Opt::new("port")
        .short("-p")
        .long("--port")
        .help("The network port to listen to."),
    )
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .render();
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", msg);
}
