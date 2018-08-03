//! Convenience wrapper to import all the essential structs.
//!
//! ```rust
//! extern crate man;
//!
//! use man::prelude::*;
//!
//! fn main () {
//!   let msg = Man::new("my-app").render();
//! }
//! ```
pub use author::Author;
pub use environment::Env;
pub use flag::Flag;
pub use man::Man;
pub use option::Opt;
