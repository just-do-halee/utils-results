# utils-results

The easiest and most intuitive error handling solution. (no dependencies, about 150 lines pure codes)  

[![Github Forks][github-forks]][github-url]
[![Github Stars][github-stars]][github-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![CI][ci-badge]][ci-url]
[![Twitter][twitter-badge]][twitter-url]

[github-forks]: https://img.shields.io/github/forks/just-do-halee/utils-results?style=flat&logo=starship&color=dec968&labelColor=383636
[github-stars]: https://img.shields.io/github/stars/just-do-halee/utils-results?style=flat&logo=starship&color=dec968&labelColor=383636
[crates-badge]: https://img.shields.io/crates/v/utils-results.svg?labelColor=383636
[ci-badge]: https://github.com/just-do-halee/utils-results/actions/workflows/ci.yml/badge.svg
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee

[twitter-url]: https://twitter.com/do_halee
[github-url]: https://github.com/just-do-halee/utils-results
[crates-url]: https://crates.io/crates/utils-results
[ci-url]: https://github.com/just-do-halee/utils-results/actions
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg?labelColor=383636
[mit-url]: https://github.com/just-do-halee/utils-results/blob/main/LICENSE
| [Docs](https://docs.rs/utils-results) |

```toml
[dependencies]
utils_results = "4.0.0"
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
| Result
```
[src/main.rs 40:1] unexpected eof. bar is 2 <err::UnexpectedEof>
```

# ***Important***

### utils-results can handle lots of errors in a beautiful one way.
### It's called **Non panic unwraping chaining Errors**.
### errbang -> errcast -> errcast -> ... ->  errcast -> errextract  
  
  
## * Quick Overview

```rust
use utils_results::*;

err! {
    One => "this error is first one."
    Two => "this error is second one."
    Three => "this error is third one."
    Well => "is this?"
}


fn aaa() -> Result<usize> {
    return errbang!(err::One, "{}.error bang!", 1);
}

fn bbb() -> Result<usize> {
    let n = errcast!(aaa(), err::Two, "{}.two <- one.", 2);
    Ok(n)
}

fn ccc() -> ResultSend<usize> { // Result -> ResultSend
    Ok(errcast!(bbb(), err::Three, "{}.three <- two.", n))
}


fn main() -> Result<()> {
    let c = errextract!(ccc(), err::Well => 127);
    eprintln!("1/{} is cosmological constant.", c);
    Ok(())
}
```

| Result
```
Error:
[src/main.rs 11:12] this error is first one. 1.error bang! <err::One> aaa()
                    ⎺↴
[src/main.rs 14:13] this error is second one. 2.two <- one. <err::Two> bbb()
                    ⎺↴
[src/main.rs 18:8] this error is third one. 3.three <- two. <err::Three>
```
If the matching error be changed,
```rust
// Well to Three
let c = errextract!(ccc(), err::Three => 127);
```
| Result
```
1/127 is cosmological constant.
```

---

# ***errcast***
Any type of error can be converted into our Master Error. **(non panic unwraping)**

##### \<Unwraped Ok\> = *errcast!* (\<Any Result\>, \<Master Err\>, \<Optional meta,..\>);

```rust
// example
let num_read = errcast!(file.read(&mut buf), err::ReadErr, "this is {} data.", "meta");
```

---
# Simply just do this!
```rust
let file = errcast!(File::open("test"), err::FileOpenError)
```
## or...
```rust
// Master `Result` can take any errors(dyn error)
let file = File::open("test")?;
```
But, *errcast* -> ***errextract*** combo is always good choice.

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
Well, we can also handle io::Error more idiomatic way.

## ***Matching `io::Error`***
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
// to floating Result
resultcast!(handle.join().unwrap())?;
```
