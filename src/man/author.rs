/// An author entry.
#[derive(Debug)]
pub struct Author {
  pub(crate) name: String,
  pub(crate) email: Option<String>,
}
