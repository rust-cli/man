/// Boolean arguments that can be toggled on or off.
#[derive(Debug, Clone)]
pub struct Flag {
  pub(crate) short: Option<String>,
  pub(crate) long: Option<String>,
  pub(crate) help: Option<String>,
}

impl Flag {
  /// Create a new instance.
  pub fn new() -> Self {
    Self {
      short: None,
      long: None,
      help: None,
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

  /// Set the help value.
  pub fn help(mut self, help: &str) -> Self {
    self.help = Some(help.into());
    self
  }
}
