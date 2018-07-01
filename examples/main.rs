extern crate clap;
extern crate man;

use clap::{App, AppSettings, Arg, SubCommand};
use man::Manual;

fn main() {
  let a = App::new("testapp")
    .about("Pointless application")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .author("Katharina Fey <kookie@spacekookie.de>")
    // .author("Yosh Wuyts <y@w.s")
    .long_about("Lorem Ipsum bla bla bla")
    .arg(Arg::with_name("debug").short("d").help("Make program output debug messages"))
    .arg(Arg::with_name("output").short("o").takes_value(true).help("Output File"))
    .subcommand(SubCommand::with_name("foo").arg(Arg::with_name("bar").short("b").long("barr")));

  let manual = Manual::from_clap(&a);
  println!("{:#?}", manual);

  // let page = Man::new("basic")
  //   .description("A basic example")
  //   .author("Alice", Some("alice@email.com"))
  //   .author("Bob", Some("bob@email.com"))
  //   .flag(Some("-d"), Some("--debug"), Some("Activate debug mode"))
  //   .flag(Some("-v"), Some("--verbose"), Some("Verbose mode"));
  //   .option(Some("-o"), Some("--output"), "output", None, "Output file");
}
