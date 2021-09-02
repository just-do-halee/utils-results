## 5.2.1 (September 3, 2021)

### A typing error.
* now no-std valid

---

## 5.2.0 (September 1, 2021)

### Apache 2.0 License added
### Refactoring features in Cargo.toml

---

## 5.1.1 (August 29, 2021)

### alloc::{`format`, string::`String`} export
* for no-std

---

## 5.1.0 (August 29, 2021)

### errunwrap!(\<Any Result\>)
* Update, Now can extract some input data and compare them. 

---

## 5.0.0 (August 29, 2021)

### Anyhow Result and Error
### etc...

---

## 4.3.1 (August 11, 2021)

### README

---

## 4.3.0 (August 9, 2021)

### More Pretty Formatting

### errpanic!(`<err::MyErr>`, ... )
* New Feature, panic! + Master Error.

### errcast_panic!(***\<Any Result\>***, ... )
* New Feature, panic! + errcast!.

---

## 4.2.1 (August 7, 2021)

### More Clear Formatting for External Error Chaining
## Example
* | io error
* Error: Custom { kind: NotFound, error: "[src/main.rs 13:5] io::Error <------------------ No such file or directory (os error 2) std::fs::File::open(\"test\")\n                    ⎺↴\n[src/main.rs 9:13] this err  `<err::MyErr>`" }

---

## 4.2.0 (August 5, 2021)

### Refactoring & Performance Improvement

### More Clear and Pretty Design Formatting

### io_to_err!(..) & err_to_io(..)
* Bug Fixed, precise file|line|column stamp.

---

## 4.1.0 (August 4, 2021)

### errcast!(\<Any Result\>)
* Update, Now can also cast by empty err::MyError Argument. 

### resultcast!(\<Any Result\>)
* Bug Fixed, Now it can convert Any Result Type to the our `Master Result<T>` only.

### resultcastsend!(\<Any Result\>)
* Restore previous version of macro. resultcastsend! can convert Any Type of Result to the our Master `ResultSend<T>`.