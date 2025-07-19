- Most Rust profiling seems awful.
- Linux maybe has the best profiling tools, Windows the worst.

### Mac

- For Mac Samply is pretty interesting: https://github.com/mstange/samply?tab=readme-ov-file
- The following from the Samply Github page produces some significant graphical output inside Brave (but it won't display with shields up)

```
The best way is the following:

1. Create a global cargo profile called profiling, see below how.
2. Compile with cargo build --profile profiling.
3. Record with samply record ./target/profiling/yourrustprogram.

To create the profiling cargo profile, create a text file at ~/.cargo/config.toml with the following content:

[profile.profiling]
inherits = "release"
debug = true
```

Use ```touch ~/.cargo/config.toml``` if the file does not exist.

- Samply displays in the browser using localhost:3000.
