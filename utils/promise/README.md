### This is a simple promise implementation with an adaptive threadpool.

Usage:

```rust
use abyss_promise::Promise;

let promise = Promise::new(|promise| {
    std::thread::sleep(std::time::Duration::from_millis(100));
    promise.resolve(0);
});

assert_eq!(promise.resolve(), Some(0));
```
