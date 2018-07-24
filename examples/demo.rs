extern crate man;

use man::Man;

fn main() {
  let msg = Man::new("auth-service")
    .description("authorize & authenticate members")
    .argument("path".into())
    .environment(
      "PORT".into(),
      None,
      Some("The network port to listen to.".into()),
    )
    .flag(
      Some("-h".into()),
      Some("--help".into()),
      Some("Prints help information.".into()),
    )
    .flag(
      Some("-V".into()),
      Some("--version".into()),
      Some("Prints version information.".into()),
    )
    .flag(
      Some("-v".into()),
      Some("--verbosity".into()),
      Some("Pass multiple times to print more information.".into()),
    )
    .option(
      Some("-a".into()),
      Some("--address".into()),
      Some("The network address to listen to.".into()),
      "address".into(),
      Some("127.0.0.1".into()),
    )
    .option(
      Some("-p".into()),
      Some("--port".into()),
      Some("The network port to listen to.".into()),
      "port".into(),
      None,
    )
    .author("Alice Person", Some("alice@person.com".into()))
    .author("Bob Human", Some("bob@human.com".into()))
    .render();
  // .option(Some("-o"), Some("--output"), "output", None, "Output file");

  println!("{}", msg);
}
