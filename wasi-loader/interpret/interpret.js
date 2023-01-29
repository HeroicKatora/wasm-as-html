import { WASI, File, PreopenDirectory } from "@bjorn3/browser_wasi_shim";
import wasm_config_module from 'cargo-wasm32:wasi_loader';
import wasm_zip_module from 'cargo-wasi:unzip';

async function loadInterpretedData(loader_data) {
  let shared = {};
  let output = new Uint8Array();

  let loader_module = await WebAssembly.compile(wasm_config_module);

  let loader = await WebAssembly.instantiate(loader_module, {
    wah_wasi: {
      length: () => loader_data.length,
      get: (ptr) => new Uint8Array(shared.memory.buffer).set(loader_data, ptr),
      put: (ptr, len) => output = new Uint8Array(shared.memory.buffer).slice(ptr, ptr+len),
    },
  });

  shared.memory = loader.exports.memory;
  loader.exports.configure();

  return new Uint32Array(output.buffer);
}

async function unzip(bin_data, configuration) {
  let unzip_module = await WebAssembly.compile(wasm_zip_module);

  let stdout = new File([]);
  let stderr = new File([]);
  let stddir = new PreopenDirectory(".", {
      "stdin": new File(bin_data[0]),
      "stdout": stdout,
      "stderr": stderr,
    });

  let outdir = new PreopenDirectory(".", {});

  let fds = [
    stddir.path_open(0, "stdin", 0).fd_obj,
    stddir.path_open(0, "stdout", 0).fd_obj,
    stddir.path_open(0, "stderr", 0).fd_obj,
    outdir,
  ];

  let args = ["unzip"];
  let env = [];

  let wasi = new WASI(args, env, fds);
  let inst = await WebAssembly.instantiate(unzip_module, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });  

  try {
    wasi.start(inst);
  } finally {
    console.log('UNZIP', outdir, stddir, wasi);
    console.log('ERROR', new TextDecoder('utf-8').decode(stderr.data));
  }

  return outdir;
}

let instructions = loadInterpretedData;

export {
  instructions,
  unzip,
};

export default loadInterpretedData;
