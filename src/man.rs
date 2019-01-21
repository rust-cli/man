use super::*;
use roff::{bold, italic, list, Roff, Troffable};

/// The main man page struct.
#[derive(Debug, Clone)]
pub struct Manual {
  name: String,
  about: Option<String>,
  description: Option<String>,
  authors: Vec<Author>,
  flags: Vec<Flag>,
  options: Vec<Opt>,
  environment: Vec<Env>,
  arguments: Vec<Arg>,
  custom_sections: Vec<Section>,
  examples: Vec<Example>,
}

impl Manual {
  /// Create a new instance.
  pub fn new(name: &str) -> Self {
    Self {
      name: name.into(),
      about: None,
      description: None,
      authors: vec![],
      flags: vec![],
      options: vec![],
      arguments: vec![],
      environment: vec![],
      custom_sections: vec![],
      examples: vec![],
    }
  }

  /// Add a short description.
  pub fn about<S: Into<String>>(mut self, about: S) -> Self {
    self.about = Some(about.into());
    self
  }

  /// Add a long description.
  pub fn description<S: Into<String>>(mut self, description: S) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Add an author.
  pub fn author(mut self, author: Author) -> Self {
    self.authors.push(author);
    self
  }

  /// Add an environment variable.
  pub fn env(mut self, env: Env) -> Self {
    self.environment.push(env);
    self
  }

  /// Add an flag.
  pub fn flag(mut self, flag: Flag) -> Self {
    self.flags.push(flag);
    self
  }

  /// Add an option.
  pub fn option(mut self, opt: Opt) -> Self {
    self.options.push(opt);
    self
  }

  /// Add a custom section
  pub fn custom(mut self, custom_section: Section) -> Self {
    self.custom_sections.push(custom_section);
    self
  }

  /// Add an examples section
  pub fn example(mut self, example: Example) -> Self {
    self.examples.push(example);
    self
  }

  /// Add a positional argument. The items are displayed in the order they're
  /// pushed.
  // TODO: make this accept argument vecs / optional args too.  `arg...`, `arg?`
  pub fn arg(mut self, arg: Arg) -> Self {
    self.arguments.push(arg);
    self
  }

  /// Render to a string.
  pub fn render(self) -> String {
    let man_num = 1;
    let mut page = Roff::new(&self.name, man_num);
    page = about(page, &self.name, &self.about);
    page = synopsis(
      page,
      &self.name,
      &self.flags,
      &self.options,
      &self.arguments,
    );
    page = description(page, &self.description);
    page = flags(page, &self.flags);
    page = options(page, &self.options);
    page = env(page, &self.environment);
    for section in self.custom_sections.into_iter() {
      page = custom(page, section);
    }
    page = exit_status(page);
    page = examples(page, &self.examples);
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
fn about(page: Roff, name: &str, desc: &Option<String>) -> Roff {
  let desc = match desc {
    Some(ref desc) => format!("{} - {}", name, desc),
    None => name.to_owned(),
  };

  page.section("NAME", &[desc])
}

/// Create a `DESCRIPTION` section.
///
/// ## Formatting
/// ```txt
/// DESCRIPTION
///         Very long description of the application
/// ```
fn description(page: Roff, desc: &Option<String>) -> Roff {
  if let Some(desc) = desc {
    page.section("DESCRIPTION", &[desc.to_owned()])
  } else {
    page
  }
}

/// Create a `SYNOPSIS` section.
fn synopsis(
  page: Roff,
  name: &str,
  flags: &[Flag],
  options: &[Opt],
  args: &[Arg],
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
    msg.push(format!(" {}", arg.name));
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
      auth_values.push(String::from("\n"));
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
      arr.push(String::from("\n\n"));
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
    args.push(italic(&opt.name));
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
      arr.push(String::from("\n\n"));
    }
  }
  page.section("OPTIONS", &arr)
}

/// Create a `ENVIRONMENT` section.
///
/// ## Formatting
///
/// ```txt
/// ENVIRONMENT
/// ```
fn env(page: Roff, environment: &[Env]) -> Roff {
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
      arr.push(String::from("\n\n"));
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
    &[
      list(&[bold("0")], &["Successful program execution.\n\n"]),
      list(&[bold("1")], &["Unsuccessful program execution.\n\n"]),
      list(&[bold("101")], &["The program panicked."]),
    ],
  )
}

/// Create a custom section.
///
/// The custom section will have the title you specify as the argument to the
/// .new() method and may optionally be followed by one or more paragraphs
/// using the .paragraph() method.
///
/// ## Formatting
/// ```txt
/// SECTION NAME
///        Text of first paragraph
///
///        Text of second paragraph
///
/// ```
fn custom(page: Roff, custom_section: Section) -> Roff {
  let mut paragraphs: Vec<String> = vec![];
  for paragraph in custom_section.paragraphs.into_iter() {
    paragraphs.push(paragraph);
    paragraphs.push("\n\n".into())
  }
  page.section(&custom_section.name, &paragraphs)
}

/// Create an examples section
///
/// examples can have text (shown before the example command) and the command
/// itself.  Optionally, you can also display the output of the command, but
/// this is typically not necessary.  You may also change the prompt displayed
/// before the command (the default is `$`).
///
/// The command is printed in bold.
///
/// ## Formatting
/// ```txt
/// EXAMPLES
///        Explanatory text
///        $ command
///        output
/// ```
fn examples(page: Roff, examples: &[Example]) -> Roff {
  if examples.is_empty() {
    return page;
  };
  let mut arr = vec![];
  for example in examples {
    let text = example.text.unwrap_or("");
    let mut full_command = String::from(example.prompt);
    if let Some(command) = example.command {
      full_command.push_str(" ");
      full_command.push_str(command);
    };
    let output = match example.output {
      Some(output) => {
        // For now, we need to manually add the line break in the list
        // see https://github.com/killercup/roff-rs/issues/5
        let mut full_output = String::from("\n.br\n");
        full_output.push_str(output);
        full_output.push_str("\n");
        full_output
      }
      None => String::from("\n"),
    };
    let example = list(&[text], &[bold(full_command.as_str()), output]);
    arr.push(example);
  }
  page.section("examples", &arr)
}

// NOTE(yw): This code was taken from the npm-install(1) command. The location
// on your system may vary. In all honesty I just copy-pasted this. We should
// probably port this to troff-rs at some point.
//
// ```sh
// $ less /usr/share/man/man1/npm-install.1
// ```
fn init_list() -> String {
  String::from(".P\n.RS 2\n.nf\n")
}
