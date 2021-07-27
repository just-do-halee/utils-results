/*
    .. + results.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

/// this will convert any result type to the another result type.
///```no_run
/// resultcast!(errbangsend!(err::UnexpectedEof), Result::<()>);
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr, $new_result:ty) => {
        match $result {
            Ok(o) => <$new_result>::Ok(o),
            Err(e) => <$new_result>::Err(e),
        }
    };
}
