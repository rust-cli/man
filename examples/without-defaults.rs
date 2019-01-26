extern crate man;

use man::prelude::*;

fn main() {
  let msg = Manual::new("auth-service")
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
    .custom(
      Section::new("custom section")
        .paragraph("text for the custom section")
        .paragraph("Additional text for the custom section"),
    )
    .custom(
      Section::new("second custom section")
        .paragraph("text for the custom section")
        .paragraph("Additional text for the custom section"),
    )
    .exit_status(
      ExitStatus::new(0).description("Successful program execution."),
    )
    .exit_status(ExitStatus::new(1).description("Invalid input."))
    .exit_status(ExitStatus::new(2).description("Could not read config file."))
    .exit_status(ExitStatus::new(3).description("Could not connect to sever."))
    .exit_status(ExitStatus::new(101).description("The program panicked."))
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
    .author(Author::new("Alice Person").email("alice@person.com"))
    .author(Author::new("Bob Human").email("bob@human.com"))
    .render();

  println!("{}", msg);
}
