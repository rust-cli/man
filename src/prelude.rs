//! Convenience wrapper to import all the essential structs.
//!
//! ```rust
//! extern crate man;
//!
//! use man::prelude::*;
//!
//! fn main () {
//!   let msg = Manual::new("my-app").render();
//! }
//! ```
pub use arg::Arg;
pub use author::Author;
pub use environment::Env;
pub use flag::Flag;
pub use man::Manual;
pub use option::Opt;
pub use section::Section;
