This describes how to wrap some Yew Apps into a self-contained HTML.

Checkout [`yew`](https://github.com/yewstack/yew). Note that we won't be making
use of `trunk`, any asset use will also not be packed. This program is _not_ a
standalone bundler! (And neither is trunk).

```bash
cd yew/examples
# Compile the wasm target module
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --release --no-default-features --target wasm32-unknown-unknown -p todomvc
# Pack with wasm-bindgen
wasm-bindgen --out-dir target/generated --web target/wasm32-unknown-unknown/release/todomvc.wasm
# Finally, create the full page
wasm-as-html --index-html todomvc/index.html \
  target/generated/todomvc.js \
  < target/generated/todomvc_bg.wasm \
  > todomvc.html
```
