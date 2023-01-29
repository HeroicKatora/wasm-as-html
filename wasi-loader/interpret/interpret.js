import { WASI, File, PreopenDirectory } from "@bjorn3/browser_wasi_shim";
import wasm_config_module from 'cargo-wasi:wasi_loader';
import wasm_zip_module from 'cargo-wasi:unzip';

async function loadInterpretedData(loader_data) {
  let loader_module = WebAssembly.compile(wasm_config_module);

  let stdin = new File(loader_data);
  let stdout = new File([]);
  let stderr = new File([]);
  let stddir = new PreopenDirectory(".", {
      "stdin": stdin,
      "stdout": stdout,
      "stderr": stderr,
  });

  let fds = [
    stddir.path_open(0, "stdin", 0).fd_obj,
    stddir.path_open(0, "stdout", 0).fd_obj,
    stddir.path_open(0, "stderr", 0).fd_obj,
    stddir,
  ];

  let wasi = new WASI(["wasi-loader-interpreter"], [], fds);
  let inst = await WebAssembly.instantiate(await loader_module, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });

  wasi.start(inst);
  return new Uint32Array(stdout.data.buffer);
}

async function unzip(bin_data, configuration) {
  let unzip_module = WebAssembly.compile(wasm_zip_module);

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

  let args = ["wasi-loader-unzip"];
  let env = [];

  let wasi = new WASI(args, env, fds);
  let inst = await WebAssembly.instantiate(await unzip_module, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });  

  wasi.start(inst);
  return outdir.dir.contents;
}

let instructions = loadInterpretedData;

export {
  instructions,
  unzip,
};

export default loadInterpretedData;
