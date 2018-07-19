/// Command line environment variable representation.
#[derive(Debug)]
pub struct Env {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) description: Option<String>,
}
