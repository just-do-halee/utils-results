# utils-results

The easiest and most intuitive error handling solution.<br>  

[![Github Forks][github-forks]][github-url]
[![Github Stars][github-stars]][github-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![CI][ci-badge]][ci-url]
[![Twitter][twitter-badge]][twitter-url]

[github-forks]: https://img.shields.io/github/forks/just-do-halee/utils-results?style=flat&logo=starship&color=dec968&labelColor=383636
[github-stars]: https://img.shields.io/github/stars/just-do-halee/utils-results?style=flat&logo=starship&color=dec968&labelColor=383636
[crates-badge]: https://img.shields.io/crates/v/utils-results.svg?labelColor=383636
[ci-badge]: https://github.com/just-do-halee/utils-results/actions/workflows/ci.yml/badge.svg
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[license-badge]: https://img.shields.io/crates/l/utils-results?labelColor=383636

[twitter-url]: https://twitter.com/do_halee
[github-url]: https://github.com/just-do-halee/utils-results
[crates-url]: https://crates.io/crates/utils-results
[ci-url]: https://github.com/just-do-halee/utils-results/actions
[license-url]: https://github.com/just-do-halee/utils-results

| [Docs](https://docs.rs/utils-results) | [Latest Note](https://github.com/just-do-halee/utils-results/blob/main/CHANGELOG.md) |

```toml
[dependencies]
utils_results = "5.2.1"
```

## No-std

Disable default feature(allocator is needed).

```toml
[dependencies]
utils_results = { version = "5.2.1", default-features = false }
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
errbang!("error.");
errbang!(err::MyError1);
errbang!(err::MyError2, "cannot find.");
errbang!(err::MyError3, "{} is {}", "bar", 2);
```
| Result
```
[src/main.rs 40:1] unexpected eof. bar is 2 <err::UnexpectedEof>
```

unwrapping error input data. also can easily compare them.
```rust
fn foo() -> Result<()> {
    // example
    return errbang!(err::Bar, "this is input.");
}

assert_eq!(
   errunwrap!(foo(), err::Bar), "this is input."
);
```

# ***Important***

- 1. One result type(`anyhow`).
- 2. All casted errors have their own chaining error' information(all the previous errors).

if you follow the below rules, you can easliy debug all your project.

### errbang -> errcast -> errcast -> ... ->  errcast -> errextract  

  
---
  
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

fn ccc() -> Result<usize> {
    Ok(errcast!(bbb(), err::Three, "{}.three <- two.", 3))
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
If the matching error has changed,
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

// if no std,
let file = io_to_err!(File::open("test"))?;
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
Well, we can also handle io::Error more idiomatic way. (feature = "std")

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
* Please use our Master ***Result***\<T\><br>
instead std::result::Result or io::Result etc..  
* this is `anyhow` Result.<br>
---
###### ***utils-results/lib.rs*** Definition
```rust
/// Master Result
pub type Result<T> = anyhow::Result<T>;
```

---
### just put this in your project.
```rust
pub use utils_results::*;
```

## You can also convert any type of `Result`
```rust
// to our Master Result
resultcast!(handle.join().unwrap())?;
```