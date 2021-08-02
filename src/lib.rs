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
//!
//! Also casted error has more information.  
//!
//! ```no_run
//! // example
//! let file = errcast!(fs::File::open("test"), err::MyError, "also io error");
//! ```
//! ```
//! Error: MyError { meta: "[src/main.rs:8] casted error [ fs::File::open(\"test\") ==> Os { code: 2, kind: NotFound, message: \"No such file or directory\" } ] *also io error", message: "this is my error." }
//! ```
//!
//! ---
//!
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
//! But, *errcast* -> ***errextract*** combo is really good choice.
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
//!
//! #### ***More idiomatic way to handle*** `io::Error`  
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
//! ```rust
//! // to our Master Result
//! resultcast!(handle.join().unwrap())?;
//! // also can convert master Result to ResultSend
//! resultcastsend!(some_master_result())?;
//! ```

use std::{error, result};

/// Master Result
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
/// Master Result for Send + Sync trait
pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

#[macro_use]
mod macros;
