# utils-results

The easiest and most intuitive error handling solution. (no dependencies, about 150 lines pure codes)


[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
![example workflow](https://github.com/just-do-halee/utils-results/actions/workflows/ci.yml/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/utils-results.svg
[crates-url]: https://crates.io/crates/utils-results
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/just-do-halee/utils-results/blob/main/LICENSE
| [Docs](https://docs.rs/utils-results) |

## Overview

First, You should make your own an error set.
```rust
err! {
     BrokenHeader => "broken header."
     AnotherHeader => "not matched header."
     FileNotFound => "file not found."
     EmptyArgument => "empty argument."
     UnexpectedEof => "unexpected eof."
     OutOfBounds => "index out of bounds."
     NotMatched => "btw not matched."
}
```
And just errbang!
```rust
errbang!(err::BrokenHeader)
```
# More Examples
```rust
fn foo() -> Result<bool> { // Our Master Result Type
    let bar = 2;
    match bar {
        0 => Ok(true),
        1 => Ok(false),
        _ => errbang!(err::NotMatched, "{} is {}", "bar", bar),
    }
}


fn main() -> Result<()> {
    let _is_bar_zero = foo()?;
    Ok(())
}
```
```rust
errbang!(err::MyError1);
errbang!(err::MyError2, "cannot find.");
errbang!(err::MyError3, "{} is {}", "bar", 2);
```
---

#### ***Idiomatic way to handle*** `io::Error`  
```rust

 io_err! {
     // io::ErrorKind => err::MyError
     UnexpectedEof => err::MyError1
     Interrupted => err::MyError2
     NotFound => err::MyError3
     // ...
 }

```
Declare matching macro and just handle that!<br>
```rust

io_to_err!(file.seek(SeekFrom::End(0)))?;

err_to_io!(my_seek(0))?;

```
---
* Please use our Master ***Result***\<T\> and ***ResultSend***\<T\>
instead std::result::Result or io::Result etc..  
---
###### ***utils-results/lib.rs***
```rust
/// Master Result
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
/// Master Result for Send + Sync trait
pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;
```
---
### just put this in your project.
```rust
pub use utils_results::*;
```
---

## You can convert any type of Result.  

```rust
// to our Master Result
resultcast!(handle.join().unwrap())?;

// also can convert master Result to ResultSend
resultcastsend!(some_master_result())?;
```
