I asked ChatGPT: "Does Rust substitute _ for - in certain instances?"

Reply:

> In Rust, hyphens (-) and underscores (_) are not interchangeable in identifiers. Rust enforces consistent naming conventions, particularly with crates and modules, which affects how names with hyphens or underscores are interpreted:

BUT... exceptions and gotchas...

### 1. Crate Names

> In ```Cargo.toml``` crates are commonly named with hyphens, like ```my-crate```.

> However, within Rust code, hyphens are not allowed in identifiers, so you must use underscores. For example, to import ```my-crate```, you would write:

```
rust
Copy code
use my_crate::some_function;
Rust automatically translates the hyphen in the crate name to an underscore in code.
```

### 3. Feature Flags in Cargo.toml

Feature flags in ```Cargo.toml``` can contain hyphens or underscores, and Rust treats them as equivalent. For example, defining a feature as ```my-feature``` can also be activated using ```my_feature```.
