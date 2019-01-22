extern crate man;

use man::prelude::*;

fn main() {
  let msg = Manual::new("auth-service")
    .about("authorize & authenticate members")
    .version(env!("CARGO_PKG_VERSION"))
    .date("February 2019")
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
    .example(
      Example::new()
        .text("listen on port 3000")
        .command("auth-service -p 3000")
        .output("now listening on port 3000"),
    )
    .example(
      Example::new()
        .text("auth-service may need to be run by root")
        .prompt("#")
        .command("auth-service"),
    )
    .custom(
      Section::new("custom section")
        .paragraph("text for the custom section")
        .paragraph("Additional text for the custom section"),
    )
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .render();
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", msg);
}
