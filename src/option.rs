/// Option
#[derive(Debug, Clone)]
pub struct Opt {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
}

impl Opt {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      default: None,
      description: None,
      short: None,
      long: None,
    }
  }

  /// Set the default value.
  pub fn default_value(mut self, default: &str) -> Self {
    self.default = Some(default.into());
    self
  }

  /// Set the description.
  pub fn description(mut self, description: &str) -> Self {
    self.description = Some(description.into());
    self
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
}
