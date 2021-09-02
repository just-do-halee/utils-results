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
//! ```no_run
//! errbang!("error.");
//! errbang!(err::MyError1);
//! errbang!(err::MyError2, "cannot find.");
//! errbang!(err::MyError3, "{} is {}", "bar", 2);
//! ```
//!
//! | Result
//! ```text
//! [src/main.rs 40:1] unexpected eof. bar is 2 <err::UnexpectedEof>
//! ```
//!
//! unwrapping error input data. also can easily compare them.
//! ```no_run
//! fn foo() -> Result<()> {
//!     // example
//!     return errbang!(err::Bar, "this is input.");
//! }
//!
//! assert_eq!(
//!    errunwrap!(foo(), err::Bar), "this is input."
//! );
//! ```
//!
//! # ***Important***
//!
//! - 1. One result type(`anyhow`).
//! - 2. All casted errors have their own chaining error' information(all the previous errors).
//!
//! if you follow the below rules, you can easliy debug all your project.
//!
//! ### errbang -> errcast -> errcast -> ... ->  errcast -> errextract  
//!   
//! ---
//!   
//! ## Quick Overview
//!
//! ```no_run
//! use utils_results::*;
//!
//! err! {
//!     One => "this error is first one."
//!     Two => "this error is second one."
//!     Three => "this error is third one."
//!     Well => "is this?"
//! }
//!
//!
//! fn aaa() -> Result<usize> {
//!     return errbang!(err::One, "{}.error bang!", 1);
//! }
//!
//! fn bbb() -> Result<usize> {
//!     let n = errcast!(aaa(), err::Two, "{}.two <- one.", 2);
//!     Ok(n)
//! }
//!
//! fn ccc() -> Result<usize> {
//!     Ok(errcast!(bbb(), err::Three, "{}.three <- two.", n))
//! }
//!
//!
//! fn main() -> Result<()> {
//!     let c = errextract!(ccc(), err::Well => 127);
//!     eprintln!("1/{} is cosmological constant.", c);
//!     Ok(())
//! }
//! ```
//!
//! | Result
//! ```text
//! Error:
//! [src/main.rs 11:12] this error is first one. 1.error bang! <err::One> aaa()
//!                     ⎺↴
//! [src/main.rs 14:13] this error is second one. 2.two <- one. <err::Two> bbb()
//!                     ⎺↴
//! [src/main.rs 18:8] this error is third one. 3.three <- two. <err::Three>
//! ```
//! If the matching error be changed,
//! ```no_run
//! // Well to Three
//! let c = errextract!(ccc(), err::Three => 127);
//! ```
//! | Result
//! ```text
//! 1/127 is cosmological constant.
//! ```
//!
//! ---
//!
//! # ***errcast***
//! Any type of error can be converted into our Master Error.
//!
//!
//! ```no_run
//! // example
//! // <Unwraped Ok> = errcast!(<Any Result>, <Master Err>, <Optional,..>);
//! let num_read = errcast!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
//! ```
//! ---
//! # Simply just do this!
//!
//! ```no_run
//! let file = errcast!(File::open("test"), err::FileOpenError)
//! ```
//! ## or...
//! ```no_run
//! // master `Result` can take any errors
//! let file = File::open("test")?;
//!
//! if no std,
//! let file = io_to_err!(File::open("test"))?;
//! ```
//! But, *errcast* -> ***errextract*** combo is always good choice.
//!
//! ```no_run
//! fn exe(path: &str) -> Result<usize> {
//!     let file = errcast!(File::open("test"), err::FileOpenError);
//!     // .....
//!     // ...
//!     Ok(num)
//! }
//!
//! fn main() -> Result<()> {
//!     /// non panic unwraping
//!     /// and specific error can return
//!     /// matching block
//!     let num = errextract!(exe(path),
//!         err::FileOpenError => 0);
//!     /// other errors will go out -> Result<T>
//!
//!     Ok(())
//! }
//! ```
//! ---
//! # ***Master Result***
//! * Please use our Master ***Result***\<T\>
//! instead std::result::Result or io::Result etc..  
//! * this is `anyhow` Result.<br>
//! ---
//! ###### ***utils-results/lib.rs*** Definition
//! ```no_run
//! /// Master Result
//! pub type Result<T> = anyhow::Result<T>;
//! ```

//! ---
//! ### just put this in your project.
//! ```no_run
//! pub use utils_results::*;
//! ```

//! ## You can also convert any type of `Result`
//! ```no_run
//! // to our Master Result
//! resultcast!(handle.join().unwrap())?;
//! ```

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate anyhow;
/// Master Result
pub use anyhow::{Error, Result};

#[macro_use]
mod macros;

extern crate alloc;

#[doc(hidden)]
pub mod private {
    pub use alloc::{format, string::String};
    pub use anyhow::{Error, Result};
    #[cfg(not(feature = "std"))]
    pub use core::fmt;
    #[cfg(feature = "std")]
    pub use std::fmt;
}
