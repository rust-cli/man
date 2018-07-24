/// Option
#[derive(Debug, Clone)]
pub struct Opt {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) argument: String,
  pub(crate) default: Option<String>,
}
