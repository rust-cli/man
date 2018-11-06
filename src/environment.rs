/// Arguments that are passed by the executing shell.
#[derive(Debug, Clone)]
pub struct Env {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) help: Option<String>,
}

impl Env {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      default: None,
      help: None,
    }
  }

  /// Set the default value.
  pub fn default_value(mut self, default: &str) -> Self {
    self.default = Some(default.into());
    self
  }

  /// Set the help.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }
}
