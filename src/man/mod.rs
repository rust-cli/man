mod author;
mod flag;
mod option;

use self::author::Author;
use self::flag::Flag;
use self::option::Opt;
use roff::{Roff, Troffable};
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
  pub fn author(&mut self, name: impl AsRef<str>, email: Option<String>) {
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

    page = description(page, &self.name, &self.description);
    page = authors(page, &self.authors);
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
#[inline]
pub fn description(page: Roff, name: &str, desc: &Option<String>) -> Roff {
  let desc = match desc {
    Some(ref desc) => format!("{} - {}", name, desc),
    None => name.to_owned(),
  };

  page.section("NAME", &[desc])
}

/// Create a `AUTHOR` or `AUTHORS` section.
///
/// ## Formatting
/// ```txt
/// AUTHORS
///          Alice Person <alice@person.com>
///          Bob Human <bob@human.com>
/// ```
#[inline]
pub fn authors(page: Roff, authors: &[Author]) -> Roff {
  let title = match authors.len() {
    0 => return page,
    1 => "AUTHOR",
    _ => "AUTHORS",
  };

  let last = authors.len() - 1;
  let mut auth_values = vec![];
  auth_values.push(init_list());
  for (index, author) in authors.iter().enumerate() {
    auth_values.push(author.name.to_owned());

    if let Some(ref email) = author.email {
      auth_values.push(format!(" <{}>", email))
    };

    if index != last {
      auth_values.push(format!("\n"));
    }
  }

  page.section(title, &auth_values)
}

// NOTE(yw): This code was taken from the npm-install(1) command. The location
// on your system may vary. In all honesty I just copy-pasted this. We should
// probably port this to troff-rs at some point.
//
// ```sh
// $ less /usr/share/man/man1/npm-install.1
// ```
#[inline]
fn init_list() -> String {
  format!(".P\n.RS 2\n.nf\n")
}
