/*
    .. + results.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

/// this will convert any result type to Master Result.
///```no_run
/// resultcast!(handle.join().unwrap())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr) => {
        Result::Ok(errcast!($result))
    };
}

/// this will convert any result type to Master ResultSend.
///```no_run
/// resultcastsend!(handle.join().unwrap())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcastsend {
    ($result:expr) => {
        ResultSend::Ok(errcast!($result))
    };
}
