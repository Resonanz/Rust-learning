## Compiling

### For size

* https://github.com/johnthagen/min-sized-rust
* https://stackoverflow.com/questions/29008127/why-are-rust-executables-so-huge

Rust binaries even for "Hello, world!" can be large. Quoting: 

    Rust uses static linking to compile its programs, meaning that all libraries required by even the simplest Hello world! program will be compiled into your executable. This also includes the Rust runtime.

I created a Linux executable Rust release using the following in ```cargo.toml```:

```
[profile.release]
opt-level = 'z'     # Optimize for size
codegen-units = 1   # Reduce number of codegen units to increase optimizations
panic = 'abort'     # Abort on panic
strip = true        # Strip symbols from binary*
```

The resultant binary was 342 kB.

### What about compression of the executable

UPX (https://github.com/upx/upx) is recommended to compress the final binary by 50-70%. UPX releases on Github are themselves about 670 kB and cross platform (Win + Linux, no Mac it seems).

I downloaded the Linux version "upx-4.2.4-amd64_linux.tar.xz" (UPX - Linux version, statically linked) and dragged the single binary file "upx" into the root folder of my Rust project. I then used UPX to compress the 342 kB binary down to 139 kB as follows:


```
./upx --best --lzma target/release/my_release_binary
```

### Automating compression

Cargo has no post precessing capability, so... create a bash script: ```touch compile_compress_run.sh``` and make executable ```chmod +x compile_compress_run.sh```. Add the following to the script:

```
#!/bin/bash
cargo build --release
./upx --best --lzma target/release/my_release_binary
target/release/my_release_binary
```

To run the script use ```./compile_compress_run.sh```.





## Cargo automatic re-compile

* https://github.com/watchexec/cargo-watch
* https://stackoverflow.com/questions/29461693/how-can-i-get-cargo-to-recompile-changed-files-automatically

Execute the following in VSCode Terminal to automatically recompile and run upon save:

```
cargo watch -q -c -w src -x run
```

For --release code, use the following:

```
cargo watch -q -c -w src -x 'run -- --release'
```


