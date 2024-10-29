## Environment variables

These can be accessed from within a Rust program. The following will print the [package] ```name``` and ```version``` fields: 

```println!("{}: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));```
