/*
    .. + errbang.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

#[doc = include_str!("../../README.md")]

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

/// any type of inside Err() can be converted<br>
/// and Ok() will be unwraped, converted err will be escaped
/// ```no_run
/// // <Unwraped Ok> = errcast!(<Any Result>, <Master Err>, <Optional,..>);
/// let num_read = errcast!(file.read(&mut buf), err::ReadErr, "cannot read.");
/// ```
#[macro_export]
macro_rules! errcast {
    ($result:expr, $kind:ty$(, $i:expr)*) => {
        match $result {
            Ok(v) => v,
            Err(_) => return errbang!($kind$(, $i)*),
        }
    };
}

/// any type of inside Err() can match this
/// ```no_run
/// if let Err(e) = some_result() {
///     // errmatch!(<Unwraped Err>, <Any Type>)
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

/// matched error returns or excutes, other errors return to outside(escape)<br>
/// and Ok() will unwrap
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

/// create custom error list
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
        @create errstruct $kind:ident $message:tt
    ) => {

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

    };
    (
            $($kind:ident => $message:tt$(,)?)*
    ) => {

        pub mod err {
            use super::*;

            err!(@create errstruct __ "external error");


            $(
                err!(@create errstruct $kind $message);
            )*

        }

    };
}

/// matching io::Error and Master Error to use casting error macros<br>
///```no_run
/// io_to_err!(file.seek(SeekFrom::End(0)))?; // <- io::Error to err
/// err_to_io!(my_seek(0))?; // <- err to io::Error
///```
///```no_run
/// io_err! {
///     // std::io::ErrorKind => err::MyError
///     UnexpectedEof => err::MyError1
///     Interrupted => err::MyError2
/// }
///```
#[macro_export]
macro_rules! io_err {
    (
        $($kind:ident => $errkind:ty$(,)?)*
    ) => {
        pub fn fn_handle_io_to_err<T>(io_error: std::io::Result<T>, meta: String) -> Result<T> {
            match io_error {
                Err(e) => match e.kind() {
                    $(
                        std::io::ErrorKind::$kind => Err(Box::new(<$errkind>::new(meta))),
                    )*
                    _ => Err(Box::new(e)),
                },
                Ok(t) => Ok(t),
            }
        }
        pub fn fn_handle_err_to_io<T>(m_error: Result<T>) -> std::io::Result<T> {
            match m_error {
                Err(e) => match e {
                    $(
                        e if errmatch!(e, $errkind) => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::$kind, format!("{:?}", e))),
                    )*
                    _ => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))),
                },
                Ok(t) => std::io::Result::Ok(t),
            }
        }
    };
}

/// casting std::io Error to Master Error matched by `io_err`
///```no_run
/// io_to_err!(file.seek(SeekFrom::End(0)))?
///```
#[macro_export]
macro_rules! io_to_err {
    (
        $ioe:expr
    ) => {
        fn_handle_io_to_err($ioe, format!("[{}:{}] io to err", file!(), line!()))
    };
}

/// casting Master Error to std::io Error matched by `io_err`
///```no_run
/// err_to_io!(my_seek(0))?
///```
#[macro_export]
macro_rules! err_to_io {
    (
        $err:expr
    ) => {
        fn_handle_err_to_io($err)
    };
}
