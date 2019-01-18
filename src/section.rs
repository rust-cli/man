/// Add a custom section
#[derive(Debug, Clone)]
pub struct Section {
  pub(crate) name: String,
  pub(crate) paragraphs: Vec<String>,
}

impl Section {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      paragraphs: vec![],
    }
  }

  pub fn paragraph(mut self, text: &str) -> Self {
    self.paragraphs.push(text.into());
    self
  }
}
