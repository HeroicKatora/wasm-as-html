import { WASI, File, PreopenDirectory } from "@bjorn3/browser_wasi_shim";
// This include is synthesized by `build.js:wasiInterpreterPlugin`.
import { instructions, unzip } from 'wasi-config:config.toml'

async function mount(promise) {
  let wasm = await WebAssembly.compileStreaming(await promise);

  var configuration = {
    args: ["bin", "-f", "hello.hq9+"],
    env: [],
    fds: {},
  };

  var stdin = new File([]);
  var stdout = new File([]);
  var stderr = new File([]);
  
  let dir = new PreopenDirectory(".", {
      "stdin": stdin,
      "stdout": stdout,
      "stderr": stderr,
      "test.hq9+": new File(new TextEncoder("utf-8").encode(`HHHH+Q`)),
    });

  configuration.fds = [
    dir.path_open(0, "stdin", 0).fd_obj,
    dir.path_open(0, "stdout", 0).fd_obj,
    dir.path_open(0, "stderr", 0).fd_obj,
    dir,
  ];

  let wah_wasi_config_data = WebAssembly.Module.customSections(wasm, 'wah_wasi_config');
  wah_wasi_config_data.unshift(new TextEncoder('utf-8').encode('{}'));

  if (wah_wasi_config_data.length > 0) {
    /* Optional: we could pre-execute this on the config data, thus yielding
     * the `output` instructions.
     **/
    let output = await instructions(wah_wasi_config_data[0]);
    let data = new Uint8Array(output.buffer);

    let inst = new Uint32Array(output.buffer);
    var iptr = 0;

    // The configuration output is 'script' in a simple, static assignment
    // scripting language. We have objects and each instruction calls one of
    // them with some arguments.
    const ops = [
      /* 0: the configuration object */
      configuration,
      /* 1: skip */ 
      (cnt) => iptr += cnt,
      /* 2: string */
      (ptr, len) => new TextDecoder('utf-8').decode(data.subarray(ptr, ptr+len)),
      /* 3: json */
      (ptr, len) => JSON.parse(output.subarray(ptr, ptr+len)),
      /* 4: integer const */
      (c) => c,
      /* 5: array */
      (ptr, len) => output.subarray(ptr, ptr+len),
      /* 6: get */
      (from, idx) => (ops[from])[ops[idx]],
      /* 7: set */
      (into, idx, what) => (ops[into])[ops[idx]] = ops[what],
      /* 8: File */
      (what) => new File(ops[what]),
      /* 9: Directory */
      (what) => new Directory(ops[what]),
      /* 10: PreopenDirectory */
      (where, what) => new PreopenDirectory(ops[where], ops[what]),
      /* 11: Directory.open */
      (dir, im_flags, path, im_oflags) => {
        return ops[dir].path_open(im_flags, ops[path], im_oflags).fd_obj;
      },
      /* 12: unzip: (binary) => PreopenDirectory */
      async (what) => await unzip(ops[what]),
      /* 13: section */
      (what) => WebAssembly.Module.customSections(wasm, ops[what])
    ];

    document.documentElement.textContent = '\n';

    try {
      while (iptr < inst.length) {
        let fn_ = ops[inst.at(iptr)];
        let acnt = inst.at(iptr+1);
        let args = inst.subarray(iptr+2, iptr+2+acnt);

        ops.push(await fn_.apply(null, args));
        iptr += 2 + acnt;
      }
    } catch (e) {
      document.documentElement.textContent += '\nOps: '+ops;
      document.documentElement.textContent += '\nError: '+e;
    }

    document.documentElement.textContent += JSON.stringify(ops, (_, v) => typeof v === 'bigint' ? v.toString() : v);
  }

  // document.documentElement.textContent = JSON.stringify(configuration);

  let args = configuration.args;
  let env = configuration.env;
  let fds = configuration.fds;
  let filesystem = configuration.fds[3];

  let wasi = new WASI(args, env, fds);

  let inst = await WebAssembly.instantiate(wasm, {
    "wasi_snapshot_preview1": wasi.wasiImport,
  });  

  try {
    wasi.start(inst);
  } finally {
    let decoder = new TextDecoder();
    console.log(decoder.decode(stdin.data));
    console.log(decoder.decode(stdout.data));
    console.log(decoder.decode(stderr.data));
  }

  if (filesystem !== undefined && (true || configuration.html)) {
    let module = filesystem.path_open(0, "index.js", 0).fd_obj;
    let blob = new Blob([module.file.data.buffer], { type: 'application/javascript' });
    let blobURL = URL.createObjectURL(blob);

    let stage3_module = (await import(blobURL));
    stage3_module.default({ stdin, stdout, stderr, filesystem });
  }
}

export default mount;
