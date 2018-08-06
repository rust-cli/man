/// Option
#[derive(Debug, Clone)]
pub struct Opt {
  pub(crate) name: String,
  pub(crate) default: Option<String>,
  pub(crate) help: Option<String>,
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
}

impl Opt {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      default: None,
      help: None,
      short: None,
      long: None,
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
