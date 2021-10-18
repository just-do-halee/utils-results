/*
    .. + results.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

/// this will convert any result type to Master Result.
///```ignore
/// resultcast!(handle.join().unwrap())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr) => {
        $crate::private::Result::<_>::Ok(errcast!($result))
    };
}
