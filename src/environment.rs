/// Command line environment variable representation.
#[derive(Debug, Clone)]
pub struct Env {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) description: Option<String>,
}

impl Env {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      default: None,
      description: None,
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
}
