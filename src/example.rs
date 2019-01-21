/// Add a examples section
#[derive(Debug, Clone, Default)]
pub struct Example {
  pub(crate) prompt: &'static str,
  pub(crate) text: Option<&'static str>,
  pub(crate) command: Option<&'static str>,
  pub(crate) output: Option<&'static str>,
}

impl Example {
  pub fn new() -> Self {
    Self {
      prompt: "$",
      text: None,
      command: None,
      output: None,
    }
  }

  pub fn prompt(mut self, prompt: &'static str) -> Self {
    self.prompt = prompt;
    self
  }

  pub fn text(mut self, text: &'static str) -> Self {
    self.text = Some(text);
    self
  }

  pub fn command(mut self, command: &'static str) -> Self {
    self.command = Some(command);
    self
  }

  pub fn output(mut self, output: &'static str) -> Self {
    self.output = Some(output);
    self
  }
}

// pub fn paragraph(mut self, text: &str) -> Self {
//   self.paragraphs.push(text.into());
//   self
// }
