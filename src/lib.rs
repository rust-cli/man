#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate clap;

use clap::{App, Arg, ArgSettings};

/// Describe an argument or option
#[derive(Debug)]
pub struct ManualArg {
  short: Option<char>,
  long: Option<String>,
  descr: Option<String>,
}

#[derive(Debug)]
pub struct Manual {
  /// Optionally children of this app
  children: Vec<(String, Manual)>,
  /// Application name
  name: Option<String>,
  /// Type description
  description: Option<String>,
  /// Type authors
  authors: Option<String>,
  /// Type flags
  flags: Vec<ManualArg>,
  /// Type options
  options: Vec<ManualArg>,
}

impl<'a, 'b, 'c> From<&'c Arg<'a, 'b>> for ManualArg {
  fn from(s: &'c Arg) -> Self {
    ManualArg {
      short: s.short,
      long: match s.long {
        Some(s) => Some(s.into()),
        _ => None,
      },
      descr: match s.help {
        Some(s) => Some(s.into()),
        _ => None,
      },
    }
  }
}

impl Manual {
  fn new() -> Self {
    Manual {
      children: Vec::new(),
      name: None,
      description: None,
      authors: None,
      flags: Vec::new(),
      options: Vec::new(),
    }
  }

  // TODO: Make this less awful
  fn add_empty_child(&mut self, name: &str) -> &mut Manual {
    self.children.push((name.into(), Manual::new()));
    let (_, ref mut manual) = self.children.last_mut().unwrap();
    manual
  }

  fn recursive(manual: &mut Manual, app: &App) {
    manual.name = app.name.clone().into();
    manual.description = app.about.map(Into::into);
    manual.authors = app.author.map(Into::into);

    let (flags, options): (Vec<ManualArg>, Vec<ManualArg>) = app
      .args
      .iter()
      .fold((Vec::new(), Vec::new()), |(mut f, mut o), i: &Arg| {
        if i.is_set(ArgSettings::TakesValue) {
          f.push(i.into());
        } else {
          o.push(i.into());
        }

        (f, o)
      });

    manual.flags = flags;
    manual.options = options;

    app.subcommands.iter().for_each(|app| {
      let _inner_name: String = app.name.clone();
      let mut inner = manual.add_empty_child(&app.name);
      Manual::recursive(&mut inner, app);
    });
  }

  pub fn from_clap<'a, 'b>(app: &App<'a, 'b>) -> Manual {
    let mut man = Manual::new();
    Manual::recursive(&mut man, app);
    man
  }
}
