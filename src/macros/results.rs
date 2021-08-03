/*
    .. + results.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

/// this will convert any result type to floating Result.
///```no_run
/// resultcast!(handle.join().unwrap())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr) => {
        match $result {
            Ok(o) => Ok(o),
            Err(e) => errbang!(err::__;@chain "{:?} {}\n                    ⎺↴", e, stringify!($result)),
        }
    };
}
