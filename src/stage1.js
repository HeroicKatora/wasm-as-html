async function init(bytes, wasm) {
  let index_html = WebAssembly.Module.customSections(wasm, 'polyglot_stage1_html');

  console.log(index_html);
  if (index_html.length) {
    document.documentElement.innerHTML = (new TextDecoder().decode(index_html[0]));
  }

  let stage2 = WebAssembly.Module.customSections(wasm, 'polyglot_stage2');

  /* This is the wasm-bindgen flavor.
       It is one module with an default export (`init`). The exported
       function can take a Promise to a Response object that resolves to the WASM module.
       Since we have it already we just create a synthetic response.
   **/
  let blob = new Blob([stage2[0]], { type: 'application/javascript' });
  let blobURL = URL.createObjectURL(blob);
  let module = (await import(blobURL));

  /** wasm-bindgen: creates one 
  */
  let wasmblob = new Blob([bytes], { type: 'application/wasm' });
  module.default(Promise.resolve(new Response(wasmblob)));
}

export default init;
