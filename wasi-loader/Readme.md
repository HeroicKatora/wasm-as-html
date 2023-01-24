This stage2 loader prepares a virtual WASI environment, based on bundled data.

Building this stage2 loader requires eslint. See `package.json`.

```bash
 ./node_modules/.bin/esbuild stage2-wasi.js --bundle --format=esm --outfile=out.js
 ```

Then use it, e.g. here on the wasi-example (see `examples/wasi` for instructions).

```bash
cargo run --bin wasm-as-html -- -o target/out.html wasi-loader/out.js examples/wasi/wasi-example.wasm
```

This generates the webpage `target/out.html`.
