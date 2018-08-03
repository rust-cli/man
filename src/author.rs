/// An author entry.
#[derive(Debug, Clone)]
pub struct Author {
  pub(crate) name: String,
  pub(crate) email: Option<String>,
}

impl Author {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      email: None,
    }
  }

  /// Set the email field.
  pub fn email(mut self, email: &str) -> Self {
    self.email = Some(email.into());
    self
  }
}
