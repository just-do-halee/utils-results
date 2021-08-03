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
//! # ***Important***
//!
//! utils-results can handle lots of errors in a beautiful one way.<br>
//! It's called **Non panic unwraping chaining Errors**.<br>
//! errbang -> errcast -> errcast -> ... ->  errcast -> errextract<br>
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
//! fn ccc() -> ResultSend<usize> { // Result -> ResultSend
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
//! Any type of error can be converted into our Master Error. **(non panic unwraping)**
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
//! Well, we can also handle io::Error more idiomatic way.
//! ## ***Matching `io::Error`***
//! ```no_run
//!  io_err! {
//!      // io::ErrorKind => err::MyError
//!      UnexpectedEof => err::MyError1
//!      Interrupted => err::MyError2
//!      NotFound => err::MyError3
//!      // ...
//!  }
//! ```
//! Declare matching macro and just handle that!<br>
//! ```no_run
//! io_to_err!(file.seek(SeekFrom::End(0)))?;
//!
//! err_to_io!(my_seek(0))?;
//! ```
//! # ***Master Result***
//! * Please use our Master ***Result***\<T\> and ***ResultSend***\<T\>
//! instead std::result::Result or io::Result etc..  
//! ---
//! ###### ***utils-results/lib.rs*** Definition
//! ```no_run
//! /// Master Result
//! pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
//!
//! /// Master Result for Send + Sync trait
//! pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;
//! ```

//! ---
//! ### just put this in your project.
//! ```no_run
//! pub use utils_results::*;
//! ```

//! ## You can also convert any type of `Result`
//! ```no_run
//! // to floating Result
//! resultcast!(handle.join().unwrap())?;
//! ```

use std::{error, result};

/// Master Result
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
/// Master Result for Send + Sync trait
pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

#[macro_use]
mod macros;
