/*
    .. + errbang.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

// #[doc = include_str!("../../README.md")]

/// make some error. floating Err(..)
/// ```ignore
/// errbang!("error.");
/// errbang!(err::MyError1);
/// errbang!(err::MyError2, "cannot find.");
/// errbang!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errbang {
    (@create $kind:ty$(, $format_str:expr$(, $val:expr )* )?$(, @stamp: $flcb:expr$(, $flc:expr)+)?$(, @chain: $eb:expr$(, $e:expr)+)?) => {
        $crate::private::Error::msg(<$kind>::new($crate::private::format!(concat!($($eb ,)?"\n"$(, $flcb)?, " {} " $(, $format_str)?, " <{}>") $($(, $e)+)?$($(, $flc)+)?, <$kind>::message() $($(, $val)*)?, stringify!($kind))))
    };
    ($format_str:literal$(, $val:expr )*) => {
        Err(errbang!(@create err::__, $format_str$(, $val )*, @stamp: "  [{} {}:{}]", file!(), line!(), column!()).into())
    };
    ($kind:ty$(, $format_str:literal$(, $val:expr )* )?) => {
        Err(errbang!(@create $kind$(, $format_str$(, $val )* )?, @stamp: "  [{} {}:{}]", file!(), line!(), column!()).into())
    };
}

/// Any type of error can be converted into our Master Error. **(non panic unwraping)**
/// ```ignore
/// // example
/// // <Unwraped Ok> = errcast!(<Any Result>, <Master Err>, <Optional,..>);
/// let num_read = errcast!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
/// ```
/// also can
/// ```ignore
/// let num_read = errcast!(file.read(&mut buf));
/// let num_read = errcast!(file.read(&mut buf), "some error.");
/// ```
#[macro_export]
macro_rules! errcast {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => return Err(errbang!(@create err::__, @stamp: "  [{} {}:{}]", file!(), line!(), column!(), @chain: "{} {}\n {:>20}⎺↴", e, stringify!($result), " ").into()),
        }
    };
    ($result:expr, $format_str:literal$(, $val:expr )*) => {
        match $result {
            Ok(v) => v,
            Err(e) => return Err(errbang!(@create err::__, $format_str $(, $val )*, @stamp: "  [{} {}:{}]", file!(), line!(), column!(), @chain: "{} {}\n {:>20}⎺↴", e, stringify!($result), " ").into()),
        }
    };
    ($result:expr, $kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        match $result {
            Ok(v) => v,
            Err(e) => return Err(errbang!(@create $kind$(, $format_str $(, $val )*)?, @stamp: "  [{} {}:{}]", file!(), line!(), column!(), @chain: "{} {}\n {:>20}⎺↴", e, stringify!($result), " ").into()),
        }
    };
}

/// any type of inside Err() can match this
/// ```ignore
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
/// ```ignore
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
/// ```ignore
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
            chain: $crate::private::String
        }

        impl $kind {
            pub fn new(chain: $crate::private::String) -> Self {
                Self { chain }
            }
            pub fn message() -> &'static str {
                $message
            }
            pub fn input_data<'a>(&'a self) -> &'a str {
                let start = self.chain.find(']').unwrap() + $message.len() + 3;
                let end = self.chain.rfind('<').unwrap() - 1;
                self.chain.get(start..end).unwrap()
            }
        }

        impl $crate::private::fmt::Display for $kind {
            fn fmt(&self, f: &mut $crate::private::fmt::Formatter<'_>) -> $crate::private::fmt::Result {
                write!(f, " {}", self.chain)
            }
        }
        impl $crate::private::fmt::Debug for $kind {
            fn fmt(&self, f: &mut $crate::private::fmt::Formatter<'_>) -> $crate::private::fmt::Result {
                write!(f, "{0}{1}{0}", "\n".repeat(2), self.chain)
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

/// unwrapping error input data.
/// ```ignore
/// fn foo() -> Result<()> {
///     return errbang!(err::Bar, "this is input.");
/// }
///
/// assert_eq!(
///    errunwrap!(foo(), err::Bar), "this is input."
/// );
///
/// ```
/// this is equal to
/// ```ignore
/// $result.unwrap_err()
///     .downcast_ref::<$kind>()
///     .unwrap()
///     .input_data()
/// ```
/// returns boolean
#[macro_export]
macro_rules! errunwrap {
    ($result:expr, $kind:ty) => {
        $result
            .unwrap_err()
            .downcast_ref::<$kind>()
            .unwrap()
            .input_data()
    };
}

/// panic! with Master Error
/// ```ignore
/// errpanic!(err::MyError1);
/// errpanic!(err::MyError2, "cannot find.");
/// errpanic!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errpanic {
    ($kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        panic!("{0}{1}{0}\n", "\n".repeat(5), errbang!(@create $kind$(, $format_str$(, $val )* )?, @stamp: "  [{} {}:{}]", file!(), line!(), column!()))
    };
}

/// println! with Master Error
/// ```ignore
/// errprint!(err::MyError1);
/// errprint!(err::MyError2, "cannot find.");
/// errprint!(err::MyError3, "{} is {}", "bar", 2);
/// ```
#[macro_export]
macro_rules! errprint {
    ($kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        println!("{0}{1}{0}\n", "\n".repeat(5), errbang!(@create $kind$(, $format_str$(, $val )* )?, @stamp: "  [{} {}:{}]", file!(), line!(), column!()))
    };
}

/// Any type of error can be converted into our Master Error. **(and unwraping)**<br>
/// `And then panic!`
/// ```ignore
/// // example
/// // <Unwraped Ok> = errcast!(<Any Result>, <Master Err>, <Optional,..>);
/// let num_read = errcast_panic!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
/// ```
/// also can
/// ```ignore
/// let num_read = errcast_panic!(file.read(&mut buf));
/// ```
#[macro_export]
macro_rules! errcast_panic {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => panic!("{0}{1}{0}\n", "\n".repeat(5), errbang!(@create err::__, @stamp: "  [{} {}:{}]", file!(), line!(), column!(), @chain: "{} {}\n {:>20}⎺↴", e, stringify!($result), " ")),
        }
    };
    ($result:expr, $kind:ty$(, $format_str:expr$(, $val:expr )* )?) => {
        match $result {
            Ok(v) => v,
            Err(e) => panic!("{0}{1}{0}\n", "\n".repeat(5), errbang!(@create $kind$(, $format_str $(, $val )*)?, @stamp: "  [{} {}:{}]", file!(), line!(), column!(), @chain: "{} {}\n {:>20}⎺↴", e, stringify!($result), " ")),
        }
    };
}

/// matching io::Error and Master Error to use casting error macros<br>
///```ignore
/// io_to_err!(file.seek(SeekFrom::End(0)))?; // <- io::Error to err
/// err_to_io!(my_seek(0))?; // <- err to io::Error
///```
///```ignore
/// io_err! {
///     // std::io::ErrorKind => err::MyError
///     UnexpectedEof => err::MyError1
///     Interrupted => err::MyError2
/// }
///```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! io_err {
    (
        $($kind:ident => $errkind:ty$(,)?)*
    ) => {
        #[doc(hidden)]
        pub fn fn_handle_io_to_err<T>(io_error: std::io::Result<T>, file: &str, line :u32, column: u32) -> $crate::private::Result<T> {
            match io_error {
                Err(e) => match e.kind() {
                    $(
                        std::io::ErrorKind::$kind => Err(errbang!(@create $errkind, "* io to err.", @stamp: "  [{} {}:{}]", file, line, column).into()),
                    )*
                    _ => Err(e.into()),
                },
                Ok(t) => Ok(t),
            }
        }
        #[doc(hidden)]
        pub fn fn_handle_err_to_io<T>(m_error: $crate::private::Result<T>, file: &str, line :u32, column: u32) -> std::io::Result<T> {
            match m_error {
                Err(e) => match e {
                    $(
                        e if errmatch!(e, $errkind) => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::$kind, format!("  [{} {}:{}] io::Error {:-<20} {}", file, line, column, "<", e))),
                    )*
                    _ => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::Other, format!("  [{} {}:{}] io::Error {:-<20} {}", file, line, column, "<", e))),
                },
                Ok(t) => std::io::Result::Ok(t),
            }
        }
    };
}

/// casting core::io Error to Master Error matched by `io_err`
///```ignore
/// io_to_err!(file.seek(SeekFrom::End(0)))?
///```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! io_to_err {
    (
        $ioe:expr
    ) => {
        fn_handle_io_to_err($ioe, file!(), line!(), column!())
    };
}

/// casting Master Error to core::io Error matched by `io_err`
///```ignore
/// err_to_io!(my_seek(0))?
///```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! err_to_io {
    (
        $err:expr
    ) => {
        fn_handle_err_to_io($err, file!(), line!(), column!())
    };
}
