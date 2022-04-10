# minesweeper
> 扫雷

[原文](https://dev.to/qongzi/series/16975)

## Native

```shell
cargo run --release
```

## Cross Compile

```shell
# Ubuntu/Debian
docker build . -t ubuntu_cross_compile -f Dockerfile
docker run --rm -ti -v $(pwd):/app ubuntu_cross_compile
```

## WebAssembly

```shell
rustup target add wasm32-unknown-unknown

cargo install wasm-server-runner
cargo serve
```
