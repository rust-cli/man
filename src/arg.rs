#[derive(Debug, Clone)]
pub struct Arg {
  pub(crate) name: String,
}

impl Arg {
  pub fn new(name: &str) -> Self {
    Self { name: name.into() }
  }
}
