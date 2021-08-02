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

```toml
[dependencies]
utils_results = "3.1.0"
```

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
  

# ***errcast***
Any type of error can be converted into our Master Error. **(non panic unwraping)**

##### \<Unwraped Ok\> = *errcast!* (\<Any Result\>, \<Master Err\>, \<Optional meta,..\>);

```rust
// example
let num_read = errcast!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
```
---

Also casted error has more information.  

```rust
// example
let file = errcast!(fs::File::open("test"), err::MyError, "also io error");
```
```
Error: MyError { meta: "[src/main.rs:8] casted error [ fs::File::open(\"test\") ==> Os { code: 2, kind: NotFound, message: \"No such file or directory\" } ] *also io error", message: "this is my error." }
```

---

# Simply just do this!

```rust
let file = errcast!(File::open("test"), err::FileOpenError)
```
## or...
```rust
// master `Result` can take any errors
let file = File::open("test")?;
```
But, *errcast* -> ***errextract*** combo is really good choice.

```rust
fn exe(path: &str) -> Result<usize> {
    let file = errcast!(File::open("test"), err::FileOpenError);
    // .....
    // ...
    Ok(num)
}

fn main() -> Result<()> {
    /// non panic unwraping
    /// and specific error can return
    /// matching block
    let num = errextract!(exe(path),
        err::FileOpenError => 0);
    /// other errors will go out -> Result<T>

    Ok(())
}
```
---

## ***More idiomatic way to handle*** `io::Error`  
```rust

 io_err! {
     // io::ErrorKind => err::MyError
     UnexpectedEof => err::MyError1
     Interrupted => err::MyError2
     NotFound => err::MyError3
     // ...
 }

```
Declare matching macro and just handle that.<br>
```rust

io_to_err!(file.seek(SeekFrom::End(0)))?;

err_to_io!(my_seek(0))?;

```
---
# ***Master Result***
* Please use our Master ***Result***\<T\> and ***ResultSend***\<T\>
instead std::result::Result or io::Result etc..  
---
###### ***utils-results/lib.rs*** Definition
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

## You can also convert any type of `Result`
#### | easy way
```rust
// to our Master Result
resultcast!(handle.join().unwrap())?;

// also can convert master Result to ResultSend
resultcastsend!(some_master_result())?;
```
