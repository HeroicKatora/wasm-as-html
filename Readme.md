A packer that adds a webpage to WASM module, making it self-hosted!

## Motivation

At the moment, Browsers can not execute WebAssembly as a native single page
app. Hopefully, this will change at some point. This post-processor allows you
to preserve the WebAssembly interpretation, unchanged, while adding an HTML
loader as a sort of polyfill instead of a native environment.

That said you could also regard it as a more general tool for WebAssembly
plugins, as a form of hypervisor. Provide a stage2 loader that emulates the
host environment within an HTML page. This could be, for instance, a WASI
environment with a file system in local storage, a simulation of a complex
native network environment, etc. Such an app can be ran natively or deployed to
a browser as a 'hardware-agnostic' alternative, from a single binary file.

## How to use it

The 'packer' acts as a filter for a WebAssembly module. It's a simple program,
if you want to explore its command line options run `wasm-as-html --help`.

```bash
# Assume you want to deploy a 'TodoMVC' app:
wasm-as-html --index-html /my/index.html /my/todomvc.js < /my/todomvc_bg.wasm > todomvc.html
```

See [examples/yew/Readme.md][examples/yew/Readme.md] for a detailed description.

Or [TodoMVC deployed on gh-pages](https://heroickatora.github.io/wasm-as-html/examples/yew/todomvc.html).

## Overview

The program inserts bootstrap sections into the WebAssembly module. These are
designed that the respective readers of other formats (html, zip, pdf)
recognize them *instead* of the WebAssembly module.

For HTML:
- A stage0 section makes the HTML parser stop after a short header, rewrites
  the main document content to a dummy page, then jumps to a module loaded from
  another section by loading that as an ES-Module. It must be the first section
  in the WebAssembly file; and it must be in a specific range of byte-lengths.
- The stage1 section takes this control and sets up a usable environment
  comparable to a single-page app. It replaces the dummy page with an initial
  page from a specially named custom section in the original module. We're free
  to run any Javascript in this module already.
- The stage2 section takes control as if some SPA module.
    - The stage2-yew case will load an application compiled, assembled, and
      packed with Yew, wasm-bindgen (or trunk if needed).
    - The stage2-wasi will now transfer control over to WASI as the main
      driver. It begins by invoking another intermediate stage program to
      control the setup of the WASI system, similar to Unix `init`.
      See [stage2](wasi-loader/Readme.md) for more.
- The default stage3 will now inspect and process the bundled zip data. This
  module takes the role of bootloader for the original module. The zip-file
  will be treated similar to an initial disk. The astute reader might instead
  use another mechanism. The author chose this as it adds transparency to the
  contents of the initial file system and its configuration files.
- The default stage4, finally, is the original WebAssembly module into which
  all these other files are packed!

For PDF [Work-In-Progress]:
- There must be a stage0 header in the first 1kB. This will pretend to open a
  binary stream element to skip over most sections. Then an original document
  is embedded.
- As stage1, the Acrobat JavaScript API might be usable but the author does not
  particular like Acrobat's software development outcomes, in non-commercial
  settings anyways. Media and GPU embeddings are just worse and also badly
  sandboxed versions of the equivalent HTML specifications; and privacy
  nightmares. Nothing was learned from Flash. Experiment on your own.

## Experimental

There's an experimental `--edit` flag. This replaces stage1 with an auto-reload
driver, which will periodically refetch the file to compare hashes. It will then
invoke the entrypoint with a response promise for the new bytes. 

## How it works

The packer takes two arguments: A WebAssembly module, and a stage 2 loader
(Javascript file) that will be given control over the page and passed a (fake)
Promise to a Response that resolves to the file's bytes. The recommended stage
2 is some web-sys implementation which passes control directly to the parsed
and instantiated module.

The technical inner workings:
* The *first* section in WASM is a custom one that parses as HTML. The trick
  from [here](https://fuzzinglabs.com/polyglot-webassembly-module-html-js-wasm/)
  implemented in a slightly more robust fashion. Note that this `stage0` is not
  of arbitrary size. Otherwise the binary encoding of the WebAssembly section's
  length will cause problems as they get parsed as HTML. This stage reads the
  first custom section with the name `wah_polyglot_stage1`.

  The stage also contains an inline link with `rel` set as `stylesheet`, in
  what is interpreted as the body. This stylesheet is applied in order to hide
  unwanted elements and display a notice in any no-JS context.

* The second section is another custom one, which is our stage1 with arbitrary
  contents. We almost immediately dispatch here from stage0 leaving behind all
  notions of HTML that we started with. We're now preparing the web page for
  the module packed as stage2. That is, loading a new skeleton with expected
  elements that the module might need to bootstrap itself.

  It reads the section named `wah_polyglot_stage1_html` as a new document
  contents, as well as the first `wah_polyglot_stage2` for the subsequent
  module. The stage will error if multiple stage2 modules are defined.

WIP: additionally, an auxiliary `.zip` file can be passed. The packer then
ensures that the result is _also_ a valid zip archive with all files intact and
such that they are accessible from the webassembly module as a custom module.
