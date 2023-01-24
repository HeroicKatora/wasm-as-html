import { WASI, File, PreopenDirectory } from "@bjorn3/browser_wasi_shim";

async function mount(promise) {
  let args = ["bin", "arg1", "arg2"];
  let env = ["FOO=bar"];
  let fds = [
    new File([]), // stdin
    new File([]), // stdout
    new File([]), // stderr
    new PreopenDirectory(".", {
      "example.c": new File(new TextEncoder("utf-8").encode(`#include "a"`)),
      "hello.rs": new File(new TextEncoder("utf-8").encode(`fn main() { println!("Hello World!"); }`)),
    }),
  ];

  let wasi = new WASI(args, env, fds);

  let wasm = await WebAssembly.compileStreaming(await promise);
  let inst = await WebAssembly.instantiate(wasm, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });  

  wasi.start(inst);
}

export default mount;
