## Can you give an example of Rust code that runs tests using assert macros?

```rust
// Example structure/enum we want to test
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down
}

// Functions we want to test
fn add(a: i32, b: i32) -> i32 {
    a + b 
}

fn make_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;  // Import items from parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_point() {
        let p1 = make_point(1, 2);
        let p2 = Point { x: 1, y: 2 };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_direction() {
        let d1 = Direction::Up;
        let d2 = Direction::Up;
        assert_eq!(d1, d2);
        assert_ne!(d1, Direction::Down);
    }

    #[test]
    fn test_with_message() {
        let value = 42;
        assert!(value > 0, "Value should be positive!");
    }

    #[test]
    #[should_panic(expected = "panic message")]
    fn test_panic() {
        panic!("panic message");
    }
}
```

Run tests with:
```bash
cargo test                    # Run all tests
cargo test test_point        # Run specific test
cargo test -- --show-output  # Show println! output
```

Common assert macros:
- `assert_eq!(left, right)` - Tests equality
- `assert_ne!(left, right)` - Tests inequality  
- `assert!(condition)` - Tests boolean condition
- `assert!(condition, "message")` - With custom message
- `debug_assert!()` - Only runs in debug builds

## You can run tests from a specific file in a few ways:

1. If it's a specific file like `math.rs`:
```bash
cargo test --test math
```

2. Or run tests that match a name pattern:
```bash
cargo test --test math::*      # All tests in math module
cargo test --test math::add    # Specific test named 'add' in math module
```

For example, if you have this structure:
```rust
// src/math.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

You can run just the tests in `math.rs` with:
```bash
cargo test --test math
```

Or you can create dedicated test files in a `tests` directory and run those specifically:
```
my_project/
├── src/
│   └── math.rs
└── tests/
    └── math_tests.rs
```

Then run:
```bash
cargo test --test math_tests
```
