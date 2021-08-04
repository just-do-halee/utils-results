/*
    .. + errbang.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

// #[doc = include_str!("../../README.md")]

/// make some error. floating Err(..)
/// ```no_run
/// errbang!(err::MyError1);
/// errbang!(err::MyError2, "cannot find.");
/// errbang!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errbang {
    (@create $kind:ty$(, $format_str:expr$(, $val:expr )* )?$(, @stamp: $flcb:expr$(, $flc:expr)+)?$(, @chain: $eb:expr$(, $e:expr)+)?) => {
        <$kind>::new(format!(concat!($($eb ,)?"\n"$(, $flcb)?, " {} " $(, $format_str)?, " <{}>") $($(, $e)+)?$($(, $flc)+)?, <$kind>::message() $($(, $val)*)?, stringify!($kind)))
    };
    ($kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        Err(errbang!(@create $kind$(, $format_str$(, $val )* )?, @stamp: "[{} {}:{}]", file!(), line!(), column!()).into())
    };
}

/// Any type of error can be converted into our Master Error. **(non panic unwraping)**
/// ```no_run
/// // example
/// // <Unwraped Ok> = errcast!(<Any Result>, <Master Err>, <Optional,..>);
/// let num_read = errcast!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
/// ```
/// also can
/// ```no_run
/// let num_read = errcast!(file.read(&mut buf));
/// ```
#[macro_export]
macro_rules! errcast {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => return Err(errbang!(@create err::__, @stamp: "[{} {}:{}]", file!(), line!(), column!(), @chain: "{:?} {}\n {:>19}⎺↴", e, stringify!($result), " ").into()),
        }
    };
    ($result:expr, $kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        match $result {
            Ok(v) => v,
            Err(e) => return Err(errbang!(@create $kind$(, $format_str $(, $val )*)?, @stamp: "[{} {}:{}]", file!(), line!(), column!(), @chain: "{:?} {}\n {:>19}⎺↴", e, stringify!($result), " ").into()),
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

/// non panic unwraping and specific error can return matching block<br>
/// other errors will go out -> Result\<T\>
/// ```no_run
/// fn exe(path: &str) -> Result<usize> {
///     let file = errcast!(File::open("test"), err::FileOpenError);
///     // .....
///     // ...
///     Ok(num)
/// }
///
/// fn main() -> Result<()> {
///     /// non panic unwraping
///     /// and specific error can return
///     /// matching block
///     let num = errextract!(exe(path),
///         err::FileOpenError => 0);
///     /// other errors will go out -> Result<T>
///
///     Ok(())
/// }
/// ```
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
        pub struct $kind {
            chain: String
        }

        impl $kind {
            pub fn new(chain: String) -> Self {
                Self { chain }
            }
            pub fn message() -> &'static str {
                $message
            }
        }

        impl std::error::Error for $kind {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                Some(self)
            }
        }
        impl std::fmt::Display for $kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.chain)
            }
        }
        impl std::fmt::Debug for $kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.chain)
            }
        }

    };
    (
            $($kind:ident => $message:tt$(,)?)*
    ) => {

        pub mod err {
            use super::*;

            #[doc(hidden)]
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
        #[doc(hidden)]
        pub fn fn_handle_io_to_err<T>(io_error: std::io::Result<T>, file: &str, line :u32, column: u32) -> Result<T> {
            match io_error {
                Err(e) => match e.kind() {
                    $(
                        std::io::ErrorKind::$kind => Err(errbang!(@create $errkind, "* io to err.", @stamp: "[{} {}:{}]", file, line, column).into()),
                    )*
                    _ => Err(e.into()),
                },
                Ok(t) => Ok(t),
            }
        }
        #[doc(hidden)]
        pub fn fn_handle_err_to_io<T>(m_error: Result<T>, file: &str, line :u32, column: u32) -> std::io::Result<T> {
            match m_error {
                Err(e) => match e {
                    $(
                        e if errmatch!(e, $errkind) => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::$kind, format!("[{} {}:{}] io::Error {:-<19} {:?}", file, line, column, "<", e))),
                    )*
                    _ => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other, format!("[{} {}:{}] io::Error {:-<19} {:?}", file, line, column, "<", e))),
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
        fn_handle_io_to_err($ioe, file!(), line!(), column!())
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
        fn_handle_err_to_io($err, file!(), line!(), column!())
    };
}
