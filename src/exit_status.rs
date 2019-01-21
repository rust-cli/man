/// Add a exit status section
#[derive(Debug, Clone, Default)]
pub struct ExitStatus {
  pub(crate) code: Option<i32>,
  pub(crate) description: Option<&'static str>,
  pub(crate) use_default_instead: bool,
}

impl ExitStatus {
  pub fn new(code: i32) -> Self {
    Self {
      code: Some(code),
      description: None,
      use_default_instead: false,
    }
  }

  pub fn default() -> Self {
    Self {
      code: None,
      description: None,
      use_default_instead: true,
    }
  }

  pub fn description(mut self, description: &'static str) -> Self {
    self.description = Some(description);
    self
  }
}
