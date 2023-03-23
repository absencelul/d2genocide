# d2genocide

This project is for educational purposes only. It's an experiment of porting over an old legacy Diablo II hack written in C++ to Rust.

### Development

Build with logger

```bash
cargo build --features=logging
```

Open game and run one of the following commands

```bash
cargo run --features=logging

# or directly execute the binary from the cl.
./target/i686-pc-windows-msvc/debug/injector.exe
```
