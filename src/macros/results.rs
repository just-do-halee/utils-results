/*
    .. + results.rs + ..
    Copyright 2021 Hwakyeom Kim(=just-do-halee)
*/

/// this will convert any result type to our Master Result.
///```no_run
/// resultcast!(handle.join().unwrap())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcast {
    ($result:expr) => {
        match $result {
            Ok(o) => Result::Ok(o),
            Err(e) => Result::Err(Box::new(<err::__>::new(format!(
                "[{}:{}] {:?}",
                file!(),
                line!(),
                e
            )))),
        }
    };
}

/// this will convert any result type to our Master ResultSend.
///```no_run
/// resultcastsend!(normal_master_result())?;
///```
/// result type cast macro
#[macro_export]
macro_rules! resultcastsend {
    ($result:expr) => {
        match $result {
            Ok(o) => ResultSend::Ok(o),
            Err(e) => ResultSend::Err(Box::new(<err::_>::new(format!(
                "[{}:{}] {:?}",
                file!(),
                line!(),
                e
            )))),
        }
    };
}
