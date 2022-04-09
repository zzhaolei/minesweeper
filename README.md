# minesweeper
> 扫雷

[原文](https://dev.to/qongzi/series/16975)

## Native

```shell
cargo run --release
```

## WebAssembly

```shell
rustup target add wasm32-unknown-unknown

cargo install wasm-server-runner
cargo serve
```
