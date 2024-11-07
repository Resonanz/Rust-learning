## Basic file structure including library modules

Here's how to create a Rust project with multiple library files:

```bash
cargo new my_project
cd my_project
```

Then create this structure:
```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs      # Your binary entry point
│   ├── lib.rs       # Main library file
│   ├── math.rs      # Additional library module
│   └── utils.rs     # Another library module
```

```rust
// lib.rs - Main library file
pub mod math;    // Declare modules
pub mod utils;

// Export commonly used items
pub use math::add;
pub use utils::helper;
```

```rust
// math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
// utils.rs
pub fn helper() -> &'static str {
    "I'm helping!"
}
```

```rust
// main.rs - Use your library modules
use my_project::{add, helper};  // Import from your lib

fn main() {
    println!("{}", add(2, 2));
    println!("{}", helper());
}
```

## More complex file structure including library modules

Here's a more complex example with nested modules, tests, and common patterns:

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── math/           # Module as directory
│   │   ├── mod.rs     # Module entry point
│   │   ├── basic.rs   # Basic operations
│   │   └── advanced.rs # Advanced operations
│   └── utils/
│       ├── mod.rs
│       ├── config.rs
│       └── helpers.rs
```

```rust
// lib.rs
pub mod math;
pub mod utils;

// Re-export common items
pub use math::basic::{add, subtract};
pub use utils::config::Config;
```

```rust
// src/math/mod.rs
pub mod basic;
pub mod advanced;

// You can also have module-level tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        assert_eq!(basic::add(2, 2), 4);
    }
}
```

```rust
// src/math/basic.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ops() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(subtract(4, 2), 2);
    }
}
```

```rust
// src/utils/mod.rs
pub mod config;
pub mod helpers;
```

```rust
// src/utils/config.rs
pub struct Config {
    name: String,
}

impl Config {
    pub fn new(name: &str) -> Self {
        Config {
            name: name.to_string(),
        }
    }
}
```

```rust
// main.rs
use my_project::{add, Config};

fn main() {
    let result = add(2, 2);
    let config = Config::new("test");
}
```

Test everything with different options:
```bash
# Run all tests
cargo test

# Run tests in specific module
cargo test math::basic

# Run specific test
cargo test math::basic::tests::test_basic_ops

# Show output of passing tests
cargo test -- --show-output
```

And in `Cargo.toml`:
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_project"    # The name other projects will use to import your lib
path = "src/lib.rs"    # Optional: specify the library entry point

[[bin]]
name = "my_project"    # The name of your binary
path = "src/main.rs"   # Optional: specify the binary entry point
```

Key features shown:
- Nested modules
- Tests at multiple levels
- Re-exports for convenience
- Module organization with both files and directories
- Separate configuration for lib and bin targets


## Can one library file access function in another library file?

Yes! Here's a simple example:

```rust
// lib.rs
pub mod math;
pub mod utils;
```

```rust
// math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```rust
// utils.rs
use crate::math::add;  // Import from another module using crate

pub fn do_math() -> i32 {
    // Use the add function from math.rs
    add(2, 2)
}
```

The key is using `use crate::` to reference other modules in your project. You can also use:
```rust
use super::*;  // Import from parent module
use self::*;   // Import from current module
```

That's all you need! Would you like to see more complex examples of module interactions?
