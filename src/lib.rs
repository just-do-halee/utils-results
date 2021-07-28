/*
    .. + lib.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

//! First, You should make your own an error set.
//! # Example
//! ```no_run
//! err! {
//!      BrokenHeader => "broken header."
//!      AnotherHeader => "not matched header."
//!      FileNotFound => "file not found."
//!      EmptyArgument => "empty argument."
//!      UnexpectedEof => "unexpected eof."
//!      OutOfBounds => "index out of bounds."
//!      NotMatched => "btw not matched."
//! }
//! ```
//! And just errbang!
//! ```no_run
//! errbang!(err::BrokenHeader);
//! ```
//! # More Examples
//! ```no_run
//! fn foo() -> Result<bool> { // Our Master Result Type
//!     let bar = 2;
//!     match bar {
//!         0 => Ok(true),
//!         1 => Ok(false),
//!         _ => errbang!(err::NotMatched, "{} is {}", "bar", bar),
//!     }
//! }
//! fn main() -> Result<()> {
//!     let _is_bar_zero = foo()?;
//!     Ok(())
//! }
//! ```

//! Please use our Master Result<T> and ResultSend<T> instead std::result::Result or io::Result etc..
//! ```no_run
//! /// Master Result
//! pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
//! /// Master Result for Send + Sync trait
//! pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;
//! ```

//! ---
//! ### just put this in your project.
//! ```rust
//! pub use utils_results::*;
//! ```
use std::{error, result};

/// Master Result
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
/// Master Result for Send + Sync trait
pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

#[macro_use]
mod macros;
