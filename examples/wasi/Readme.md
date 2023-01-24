Prepare the WASI binary:

```bash
rustup target install wasm32-wasi
git clone https://github.com/wapm-packages/rust-wasi-example

cd rust-wasi-example

cargo build --target=wasm32-wasi --release
cp target/wasm32-unknown-wasi/release/wasi-example.wasm ..
```

You'll find a `wasi-example.wasm` in _this_ folder.
