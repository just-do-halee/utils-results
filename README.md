# utils-results

The easiest and most intuitive error handling solution.


[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/utils-results.svg
[crates-url]: https://crates.io/crates/utils-results
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/just-do-halee/utils-results/LICENSE
| [Docs](https://docs.rs/utils-results/latest/utils-results) |

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
errbang!(err:BrokenHeader)
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
}
```
```rust
errbang!(err::MyError1);
errbang!(err::MyError2, "cannot find.");
errbang!(err::MyError3, "{} is {}", "bar", 2);
```

* Please use our Master ***Result***\<T\> and ***ResultSend***\<T\>
instead std::result::Result or io::Result etc..