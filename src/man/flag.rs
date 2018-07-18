/// Command line flag representation.
#[derive(Debug)]
pub struct Flag {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) description: Option<String>,
}
