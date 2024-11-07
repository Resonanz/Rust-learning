## Can you include code that only runs when compiling for debug? Yes !!!

You can use the `cfg` attribute with `debug_assertions` to run code only in debug builds:

```rust
fn my_function() {
    // This code always runs
    let x = 5;

    #[cfg(debug_assertions)]
    println!("Debug only: x = {}", x);  // Only runs in debug build

    // Or using if... just to be clear, this code is standalone and works as-is:
    if cfg!(debug_assertions) {
        println!("Also debug only!");
    }
}
```

When you build:
```bash
cargo build            # Debug build - code runs
cargo build --release  # Release build - code is removed
```

This is commonly used for debug logging, performance checks, or extra validation that you don't want in production code.
