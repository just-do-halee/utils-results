/*
    .. + errbang.rs + ..
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
//! errbang!(err:BrokenHeader);
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

/// make some error. Master Result::Err()
/// ```no_run
/// errbang!(err::MyError1);
/// errbang!(err::MyError2, "cannot find.");
/// errbang!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errbang {
    ($kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        Result::Err(Box::new(<$kind>::new(format!(concat!("[{}:{}] ", $($format_str)?), file!(), line!(), $( $($val),* )?))))
    };
}

/// make some error for thread. Master ResultSend::Err()
/// ```no_run
/// errbangsend!(err::MyError1);
/// errbangsend!(err::MyError2, "cannot find.");
/// errbangsend!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errbangsend {
    ($kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        ResultSend::Err(Box::new(<$kind>::new(format!(concat!("[{}:{}] ", $($format_str)?), file!(), line!(), $( $($val),* )?))))
    };
}

/// any type of inside Err() can match this
/// ```no_run
/// if let Err(e) = some_result() {
///     if errmatch!(e, err::MyError0) {
///         // ...   
///     }
/// }
/// ```
/// returns boolean
#[macro_export]
macro_rules! errmatch {
    ($err:expr, $kind:ty) => {
        match $err.downcast_ref::<$kind>() {
            Some(_) => true,
            None => false,
        }
    };
}

/// matched error returns or excutes, other errors return to outside<br>
/// and Ok(v) will unwrap and to be v
///```no_run
/// fn main() -> Result<()> {
///     let num_read = errextract!(read(),
///         err::UnexpectedEof => 0,
///     );
///     Ok(())
/// }
///```
#[macro_export]
macro_rules! errextract {
    ($result:expr, $kind:ty => $match:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) if errmatch!(e, $kind) => $match,
            Err(e) => return Err(e),
        }
    };
}

/// create custom errors list
/// ```no_run
/// err! {
///      BrokenHeader => "broken header."
///      AnotherHeader => "not matched header."
///      FileNotFound => "file not found."
///      EmptyArgument => "empty argument."
///      UnexpectedEof => "unexpected eof."
///      OutOfBounds => "index out of bounds."
///      NotMatched => "btw not matched."
/// }
///
/// errbang!(err::BrokenHeader);
/// ```
#[macro_export]
macro_rules! err {
    (
            $($kind:ident => $message:tt$(,)?)*
    ) => {

        pub mod err {

            $(
                #[derive(Debug)]
                pub struct $kind {
                    meta: String,
                    message: &'static str,
                }

                impl $kind {
                    pub fn new(meta: String) -> Self {
                        Self { meta, message: $message }
                    }
                    pub fn as_combination(&self) -> String {
                        format!("{} {}", self.meta, self.message)
                    }
                }

                impl std::error::Error for $kind {
                    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                        Some(self)
                    }
                }
                impl std::fmt::Display for $kind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.as_combination())
                    }
                }

            )*

        }
    };
}

/// casting std::io Error to Master Error (custom list)
///```no_run
/// fn_handle_io_error! {
///     // std::io::ErrorKind => err::MyError
///     UnexpectedEof => err::MyError1
///     Interrupted => err::MyError2
/// }
///```
/// and using in anywhere
///```rust
/// fn_handle_io_error(file.seek(SeekFrom::End(0)))?
///```
#[macro_export]
macro_rules! fn_handle_io_error {
    (
        $($kind:ident => $errkind:ty$(,)?)*
    ) => {
        pub fn fn_handle_io_error<T>(io_error: std::io::Result<T>) -> Result<T> {
            match io_error {
                Err(e) => match e.kind() {
                    $(
                        std::io::ErrorKind::UnexpectedEof => errbang!(err::UnexpectedEof),
                    )*
                    _ => Err(Box::new(e)),
                },
                Ok(t) => Ok(t),
            }
        }
    };
}
