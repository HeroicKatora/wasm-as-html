export default async function(configuration) {
  /* Problem statement:
   * We'd like to solve the problem of exporting our current WASI for use by
   * wasm-bindgen. It is not currently supported to pass such additional
   * imports as a parameter to the init function of wasm-bindgen. Instead, the
   * module generated looks like so:
   *
   *     import * as __wbg_star0 from 'wasi_snapshot_preview1';
   *     // etc.
   *     imports['wasi_snapshot_preview1'] = __wbg_star0;
   *
   * Okay. So can we setup such that the above `wasi_snapshot_preview1` module
   * refers to some shim that we control? Not so easy. We can not simply create
   * an importmap; we're already running in Js context and it's forbidden to
   * modify after that (with some funny interaction when rewriting the whole
   * document where warnings are swallowed?). See `__not_working_via_importmap`.
   *
   * Instead, we will hackishly rewrite the bindgen import if we're asked to.
   * Create a shim module that exports the wasi objects' import map, and
   * communicate with the shim module via a global for lack of better ideas. I
   * don't like that we can not reverse this, the module is cached, but oh
   * well. Let's hope for wasm-bindgen to cave at some point. Or the browser
   * but 'Chromium does not have the bandwidth' to implement the dynamic remap
   * feature already in much smaller products. And apparently that is the
   * motivation not to move forward in WICG. Just ____ off. When talking about
   * Chrome monopoly leading to bad outcomes, this is one. And no one in
   * particular is at fault of course.
   */
  console.log('Reached stage3 successfully', configuration);
  const wasm = configuration.wasm_module;

  let newWasi = new configuration.WASI(configuration.args, configuration.env, configuration.fds);
  document.__wah_wasi_imports = newWasi.wasiImport;

  let testmodule = Object.keys(document.__wah_wasi_imports)
    .map((name, _) => `export const ${name} = document.__wah_wasi_imports.${name};`)
    .join('\n');
  let wasi_blob = new Blob([testmodule], { type: 'application/javascript' });
  let objecturl = URL.createObjectURL(wasi_blob);

  const bindgens = WebAssembly.Module.customSections(wasm, 'wah_polyglot_wasm_bindgen');
  const wbg_source = new TextDecoder().decode(bindgens[0]).replace('wasi_snapshot_preview1', objecturl);

  let wbg_blob = new Blob([wbg_source], { type: 'application/javascript' });
  let wbg_url = URL.createObjectURL(wbg_blob);
  const m = await import(wbg_url);

  const index_html = WebAssembly.Module.customSections(wasm, 'wah_polyglot_stage1_html');
  document.documentElement.innerHTML = (new TextDecoder().decode(index_html[0]));

  const rootdir = configuration.fds[3];
  configuration.fds[0] = rootdir.path_open(0, "proc/0/fd/0", 0).fd_obj;
  configuration.fds[1] = rootdir.path_open(0, "proc/0/fd/1", 0).fd_obj;
  configuration.fds[2] = rootdir.path_open(0, "proc/0/fd/2", 0).fd_obj;
  configuration.args.length = 0;
  configuration.args.push("scene-viewer");
  configuration.args.push("default-scene/scene.gltf");

  try {
    console.log('start', configuration);

    var source_headers = {};
    const wasmblob = new Blob([configuration.wasm], { type: 'application/wasm' });
    const ret = await m.default(Promise.resolve(new Response(wasmblob, {
      'headers': source_headers,
    })));

    newWasi.start({ 'exports': ret });
    console.log('done');
  } catch (e) {
    console.log(e);
    console.log('at ', e.fileName, e.lineNumber, e.columnNumber);
    console.log(e.stack);
    throw e;
  } finally {
    const [stdin, stdout, stderr] = configuration.fds;
    console.log('Result(stdin )', new TextDecoder().decode(stdin.file.data));
    console.log('Result(stdout)', new TextDecoder().decode(stdout.file.data));
    console.log('Result(stderr)', new TextDecoder().decode(stderr.file.data));
  }
}

// The concept that did not work, importmap must not be modified/added from a script.
function __not_working_via_importmap(objecturl) {
  if (!(HTMLScriptElement.supports?.("importmap"))) {
    throw "Browser must support import maps.";
  }

  const importmap = JSON.stringify({
    "imports": {
      "testing": objecturl,
    },
  });

  const d = document.createElement('script');
  d.type = 'importmap';
  d.innerText = importmap;

  document.documentElement.innerHTML = `<head></head>`;
  document.head.appendChild(d);
}
