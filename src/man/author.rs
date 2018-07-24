/// An author entry.
#[derive(Debug, Clone)]
pub struct Author {
  pub(crate) name: String,
  pub(crate) email: Option<String>,
}
