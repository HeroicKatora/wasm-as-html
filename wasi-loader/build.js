import * as esbuild from 'esbuild'

import { execFile } from 'node:child_process';
import fs from 'node:fs';

let cratePlugin = {
  name: 'wasm32-crate',
  setup(build) {
    // Use a reference to `Cargo.toml` to denote the target
    build.onResolve({ filter: /^cargo-wasm32:.*Cargo.toml$/ }, args => ({
      path: args.path,
      namespace: 'cargo-crate-ns',
    }))

    // Invokes the cargo build and copies the output to a binary include.
    // FIXME: this is hard coded against the `wasi_loader` crate.
    build.onLoad({ filter: /.*/, namespace: 'cargo-crate-ns' }, async () => {
      execFile('cargo', ['build', '--release', '--target', 'wasm32-unknown-unknown']);
      let contents = await fs.promises.readFile('../target/wasm32-unknown-unknown/release/wasi_loader.wasm');
      let bytes = new Uint8Array(contents.buffer);

      return {
        contents: bytes,
        loader: 'binary',
      };
    })
  },
}

// This plugin analyzes our DSL to resolve any JS dependencies needed to setup
// the environment. It then resolves to a module which exports the instructions
// and dependencies accordingly.
//
// In particular, the instruction loader can be either static or dynamic. And
// dependencies are only included if they are actually used.
let wasiInterpreterPlugin = {
  name: 'wasi-interpreter',
  setup(build) {
    build.onResolve({ filter: /^wasi-config:.*$/ }, args => ({
      path: args.path + '.mjs',
      namespace: 'wasi-interpreter-ns',
    }))

    // FIXME: we want to setup an es module with 'dynamic' requirements, based
    // on the instructions that are being used in the DSL program.
    build.onLoad({ filter: /.*/, namespace: 'wasi-interpreter-ns' }, async () => {
      return {
        contents: await fs.promises.readFile('interpret.js'),
        loader: 'js',
      };
    })
  },
}

await esbuild.build({
  entryPoints: ['stage2-wasi.js'],
  bundle: true,
  outfile: 'out.js',
  format: 'esm',
  plugins: [cratePlugin],
})
