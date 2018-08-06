mod author;
mod environment;
mod flag;
mod option;

use self::author::Author;
use self::environment::Env;
use self::flag::Flag;
use self::option::Opt;
use roff::{bold, italic, list, Roff, Troffable};
use std::convert::AsRef;

/// Man page struct.
#[derive(Debug, Clone)]
pub struct Man {
  name: String,
  help: Option<String>,
  authors: Vec<Author>,
  flags: Vec<Flag>,
  options: Vec<Opt>,
  environment: Vec<Env>,
  arguments: Vec<String>,
}

impl Man {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      help: None,
      authors: vec![],
      flags: vec![],
      options: vec![],
      arguments: vec![],
      environment: vec![],
    }
  }

  /// Add a help.
  pub fn help(mut self, desc: &str) -> Self {
    let desc = desc.into();
    self.help = Some(desc);
    self
  }

  /// Add an author.
  pub fn author(
    mut self,
    name: impl AsRef<str>,
    email: Option<String>,
  ) -> Self {
    self.authors.push(Author {
      name: name.as_ref().to_owned(),
      email,
    });
    self
  }

  /// Add an environment variable.
  pub fn environment(
    mut self,
    name: String,
    default: Option<String>,
    help: Option<String>,
  ) -> Self {
    self.environment.push(Env {
      name,
      default,
      help,
    });
    self
  }

  /// Add an flag.
  pub fn flag(
    mut self,
    short: Option<String>,
    long: Option<String>,
    help: Option<String>,
  ) -> Self {
    self.flags.push(Flag {
      short,
      long,
      help,
    });
    self
  }

  /// Add an option.
  pub fn option(
    mut self,
    short: Option<String>,
    long: Option<String>,
    help: Option<String>,
    argument: String,
    default: Option<String>,
  ) -> Self {
    self.options.push(Opt {
      short,
      long,
      help,
      argument,
      default,
    });
    self
  }

  /// Add a positional argument. The items are displayed in the order they're
  /// pushed.
  // TODO: make this accept argument vecs / optional args too.  `arg...`, `arg?`
  pub fn argument(mut self, arg: String) -> Self {
    self.arguments.push(arg);
    self
  }

  pub fn render(self) -> String {
    let man_num = 1;
    let mut page = Roff::new(&self.name, man_num);
    page = help(page, &self.name, &self.help);
    page = synopsis(
      page,
      &self.name,
      &self.flags,
      &self.options,
      &self.arguments,
    );
    page = flags(page, &self.flags);
    page = options(page, &self.options);
    page = environment(page, &self.environment);
    page = exit_status(page);
    page = authors(page, &self.authors);
    page.render()
  }
}

/// Create a `NAME` section.
///
/// ## Formatting
/// ```txt
/// NAME
///         mycmd - brief help of the application
/// ```
fn help(page: Roff, name: &str, desc: &Option<String>) -> Roff {
  let desc = match desc {
    Some(ref desc) => format!("{} - {}", name, desc),
    None => name.to_owned(),
  };

  page.section("NAME", &[desc])
}

/// Create a `SYNOPSIS` section.
fn synopsis(
  page: Roff,
  name: &str,
  flags: &[Flag],
  options: &[Opt],
  args: &[String],
) -> Roff {
  let flags = match flags.len() {
    0 => "".into(),
    _ => " [FLAGS]".into(),
  };
  let options = match options.len() {
    0 => "".into(),
    _ => " [OPTIONS]".into(),
  };

  let mut msg = vec![];
  msg.push(bold(name));
  msg.push(flags);
  msg.push(options);

  for arg in args {
    msg.push(format!(" {}", arg));
  }

  page.section("SYNOPSIS", &msg)
}

/// Create a `AUTHOR` or `AUTHORS` section.
///
/// ## Formatting
/// ```txt
/// AUTHORS
///          Alice Person <alice@person.com>
///          Bob Human <bob@human.com>
/// ```
fn authors(page: Roff, authors: &[Author]) -> Roff {
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

/// Create a `FLAGS` section.
///
/// ## Formatting
/// ```txt
/// FLAGS
/// ```
fn flags(page: Roff, flags: &[Flag]) -> Roff {
  if flags.is_empty() {
    return page;
  }

  let last = flags.len() - 1;
  let mut arr: Vec<String> = vec![];
  for (index, flag) in flags.iter().enumerate() {
    let mut args: Vec<String> = vec![];
    if let Some(ref short) = flag.short {
      args.push(bold(&short));
    }
    if let Some(ref long) = flag.long {
      if !args.is_empty() {
        args.push(", ".to_string());
      }
      args.push(bold(&long));
    }
    let desc = match flag.help {
      Some(ref desc) => desc.to_string(),
      None => "".to_string(),
    };
    arr.push(list(&args, &[desc]));

    if index != last {
      arr.push(format!("\n\n"));
    }
  }
  page.section("FLAGS", &arr)
}

/// Create a `OPTIONS` section.
///
/// ## Formatting
/// ```txt
/// OPTIONS
/// ```
fn options(page: Roff, options: &[Opt]) -> Roff {
  if options.is_empty() {
    return page;
  }

  let last = options.len() - 1;
  let mut arr: Vec<String> = vec![];
  for (index, opt) in options.iter().enumerate() {
    let mut args: Vec<String> = vec![];
    if let Some(ref short) = opt.short {
      args.push(bold(&short));
    }
    if let Some(ref long) = opt.long {
      if !args.is_empty() {
        args.push(", ".to_string());
      }
      args.push(bold(&long));
    }
    args.push("=".into());
    args.push(italic(&opt.argument));
    if let Some(ref default) = opt.default {
      if !args.is_empty() {
        args.push(" ".to_string());
      }
      args.push("[".into());
      args.push("default:".into());
      args.push(" ".into());
      args.push(italic(&default));
      args.push("]".into());
    }
    let desc = match opt.help {
      Some(ref desc) => desc.to_string(),
      None => "".to_string(),
    };
    arr.push(list(&args, &[desc]));

    if index != last {
      arr.push(format!("\n\n"));
    }
  }
  page.section("OPTIONS", &arr)
}

/// Create a `ENVIRONMENT` section.
///
/// ## Formatting
/// ```txt
/// ENVIRONMENT
/// ```
fn environment(page: Roff, environment: &[Env]) -> Roff {
  if environment.is_empty() {
    return page;
  }

  let last = environment.len() - 1;
  let mut arr: Vec<String> = vec![];
  for (index, env) in environment.iter().enumerate() {
    let mut args: Vec<String> = vec![];
    args.push(bold(&env.name));
    if let Some(ref default) = env.default {
      if !args.is_empty() {
        args.push(" ".to_string());
      }
      args.push("[".into());
      args.push("default:".into());
      args.push(" ".into());
      args.push(italic(&default));
      args.push("]".into());
    }
    let desc = match env.help {
      Some(ref desc) => desc.to_string(),
      None => "".to_string(),
    };
    arr.push(list(&args, &[desc]));

    if index != last {
      arr.push(format!("\n\n"));
    }
  }
  page.section("ENVIRONMENT", &arr)
}

/// Create a `EXIT STATUS` section.
///
/// ## Implementation Note
/// This currently only returns the status code `0`, and takes no arguments. We
/// should let it take arguments.
///
/// ## Formatting
/// ```txt
/// EXIT STATUS
///        0      Successful program execution
///
///        1      Usage, syntax or configuration file error
///
///        2      Optional error
/// ```
fn exit_status(page: Roff) -> Roff {
  page.section(
    "EXIT STATUS",
    &[list(&[bold("0")], &["Successful program execution."])],
  )
}

// NOTE(yw): This code was taken from the npm-install(1) command. The location
// on your system may vary. In all honesty I just copy-pasted this. We should
// probably port this to troff-rs at some point.
//
// ```sh
// $ less /usr/share/man/man1/npm-install.1
// ```
fn init_list() -> String {
  format!(".P\n.RS 2\n.nf\n")
}
