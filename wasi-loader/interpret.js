import wasm_config_module from 'cargo-wasm32:wasi_loader';
import zip from 'cargo-wasm32:unzip';

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

let instructions = loadInterpretedData;
let unzip = undefined;

export {
  instructions,
  unzip,
};

export default loadInterpretedData;
