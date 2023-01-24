import { WASI, File, PreopenDirectory } from "@bjorn3/browser_wasi_shim";

async function mount(promise) {
  let args = ["hq9+", "-f", "test.hq9+"];
  let env = ["FOO=bar"];

  var stdin = new File([]);
  var stdout = new File([]);
  var stderr = new File([]);
  
  let dir = new PreopenDirectory(".", {
      "stdin": stdin,
      "stdout": stdout,
      "stderr": stderr,
      "test.hq9+": new File(new TextEncoder("utf-8").encode(`HHHH+Q`)),
    });

  let fds = [
    dir.path_open(0, "stdin", 0).fd_obj,
    dir.path_open(0, "stdout", 0).fd_obj,
    dir.path_open(0, "stderr", 0).fd_obj,
    dir,
  ];

  let wasi = new WASI(args, env, fds);

  let wasm = await WebAssembly.compileStreaming(await promise);
  let inst = await WebAssembly.instantiate(wasm, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });  

  try {
    wasi.start(inst);
  } finally {
    let decoder = new TextDecoder();
    console.log(stdin, stdout, stderr);
    console.log(decoder.decode(stdin.data));
    console.log(decoder.decode(stdout.data));
    console.log(decoder.decode(stderr.data));
  }
}

export default mount;
