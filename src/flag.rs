/// Command line flag representation.
#[derive(Debug, Clone)]
pub struct Flag {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) description: Option<String>,
}

impl Flag {
  /// Create a new instance.
  pub fn new() -> Self {
    Self {
      short: None,
      long: None,
      description: None,
    }
  }

  /// Set the short value.
  pub fn short(mut self, short: &str) -> Self {
    self.short = Some(short.into());
    self
  }

  /// Set the long value.
  pub fn long(mut self, long: &str) -> Self {
    self.long = Some(long.into());
    self
  }

  /// Set the description value.
  pub fn description(mut self, description: &str) -> Self {
    self.description = Some(description.into());
    self
  }
}
