## 4.1.0 (August 4, 2021)

### errcast!(\<Any Result\>)
* Update, Now can also cast by empty err::MyError Argument. 

### resultcast!(\<Any Result\>)
* Bug Fixed, Now it can convert Any Result Type to the our `Master Result<T>` only.

### resultcastsend!(\<Any Result\>)
* Restore previous version of macro. resultcastsend! can convert Any Type of Result to the our Master `ResultSend<T>`.