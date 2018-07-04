mod author;
mod flag;
mod option;

use self::author::Author;
use self::flag::Flag;
use self::option::Opt;
use roff::{self, Roff, Troffable};
use std::convert::AsRef;

/// Man page struct.
#[derive(Debug)]
pub struct Man {
  name: String,
  description: Option<String>,
  authors: Vec<Author>,
  flags: Vec<Flag>,
  options: Vec<Opt>,
}

impl Man {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      description: None,
      authors: vec![],
      flags: vec![],
      options: vec![],
    }
  }

  /// Add a description.
  pub fn description(&mut self, desc: &str) {
    let desc = desc.into();
    self.description = Some(desc);
  }

  /// Add an author.
  pub fn author(&mut self, name: impl AsRef<String>, email: Option<String>) {
    self.authors.push(Author {
      name: name.as_ref().to_owned(),
      email,
    });
  }

  /// Add an flag.
  pub fn flag(&mut self, flag: Flag) {
    self.flags.push(flag);
  }

  /// Add an option.
  pub fn option(&mut self, option: Opt) {
    self.options.push(option);
  }

  pub fn render(&mut self) -> String {
    let man_num = 1;
    let mut page = Roff::new(&self.name, man_num);

    page = description(page, &self.description);
    page.render()
  }
}

/// Create a `NAME` section.
///
/// ## Formatting
/// ```txt
/// NAME
///         mycmd - brief description of the application
/// ```
pub fn description(page: Roff, desc: &Option<String>) -> Roff {
  let desc = match desc {
    Some(ref desc) => format!("- {:?}", desc),
    None => String::from(""),
  };

  page.section("NAME", &[desc])
}

/// Create a `AUTHOR` or `AUTHORS` section.
///
/// ## Formatting
/// ```txt
/// AUTHORS
///           alice person <alice@person.com>
///           bob human <bob@human.com>
/// ```
pub fn authors(page: Roff, authors: &[Author]) -> Roff {
  let title = match authors.len() {
    0 => return page,
    1 => "AUTHOR",
    _ => "AUTHORS",
  };

  let auth_values = vec![];
  for author in authors.iter() {
    let email = match author.email {
      Some(email) => format!("- {:?}", email),
      None => "".into(),
    };
    auth_values.push([roff::bold(&author.name), email]);
  }

  page.section(title, &auth_values)
}
