## Documentation comments (often called "doc comments")

In Rust, documentation comments (often called "doc comments") are created using `///` for outer documentation and `//!` for inner documentation. Let me show you the common patterns and best practices.

```rust
//! This is a crate-level documentation comment.
//! It describes the entire crate and appears on the root module.
//! 
//! # Examples
//! 
//! ```
//! let person = your_crate::Person::new("Alice", 30);
//! assert_eq!(person.name(), "Alice");
//! ```

/// Represents a person with a name and age.
/// 
/// # Examples
/// 
/// ```
/// let person = Person::new("Bob", 25);
/// assert_eq!(person.age(), 25);
/// ```
/// 
/// # Arguments
/// 
/// * `name` - The person's name
/// * `age` - The person's age in years
/// 
/// # Panics
/// 
/// Will panic if age is greater than 150.
#[derive(Debug)]
pub struct Person {
    /// The person's full name
    name: String,
    /// The person's age in years
    age: u32,
}

impl Person {
    /// Creates a new Person instance.
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string slice containing the person's name
    /// * `age` - The person's age in years
    /// 
    /// # Returns
    /// 
    /// A new `Person` instance
    /// 
    /// # Examples
    /// 
    /// ```
    /// let person = Person::new("Charlie", 35);
    /// ```
    pub fn new(name: &str, age: u32) -> Self {
        assert!(age <= 150, "Age must be 150 or less");
        Person {
            name: name.to_string(),
            age,
        }
    }

    /// Returns the person's name.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let person = Person::new("Dave", 40);
    /// assert_eq!(person.name(), "Dave");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the person's age.
    /// 
    /// # Safety
    /// 
    /// This method is safe because age is validated during construction.
    pub fn age(&self) -> u32 {
        self.age
    }
}

// Regular comments (not documentation) start with //
// They won't appear in generated documentation

/* 
   Multi-line regular comments
   use this syntax
*/

/// This enum demonstrates documenting variants
/// 
/// # Examples
/// 
/// ```
/// use your_crate::Status;
/// let status = Status::Active;
/// ```
pub enum Status {
    /// Indicates the item is active
    Active,
    /// Indicates the item is inactive
    /// with an optional reason
    Inactive(String),
}

/// This trait demonstrates documenting trait definitions
/// 
/// Implementors should ensure that all methods are
/// thread-safe.
pub trait Processable {
    /// Processes the implementing type
    /// 
    /// # Errors
    /// 
    /// Returns an error if processing fails
    fn process(&self) -> Result<(), String>;
}

```

Here are the key points about Rust documentation:

1. Types of doc comments:
   - `///` for documenting items (functions, structs, etc.)
   - `//!` for documenting the containing item (usually the module or crate)

2. Common documentation sections:
   - `# Examples` - For code examples
   - `# Arguments` - For parameter descriptions
   - `# Returns` - For return value descriptions
   - `# Errors` - For error conditions
   - `# Panics` - For panic conditions
   - `# Safety` - For safety considerations

3. Documentation testing:
   - Code blocks in documentation (written between triple backticks) are automatically tested when running `cargo test`
   - This ensures your examples stay up to date with your code

4. Generating documentation:
   - Run `cargo doc` to generate HTML documentation
   - Run `cargo doc --open` to generate and open the documentation in your browser

5. Markdown support:
   - Documentation comments support Markdown formatting
   - You can use lists, code blocks, links, and other Markdown features

Would you like me to show you any specific documentation patterns or explain more about how to format certain types of documentation?
