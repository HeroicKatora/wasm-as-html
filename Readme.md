A packer that turns a WASM file into a webpage that can execute it!

## How to use it

The 'packer' acts as a filter for a WebAssembly module. It's a standard clap
app if you want to explore its command line options.

```bash
# Assume you want to deploy a 'TodoMVC' app:
wasm-as-html --index-html /my/index.html /my/todomvc.js < /my/todomvc_bg.wasm > todomvc.html
```

See [examples/yew/Readme.md] for a specific description.

Or [TodoMVC deployed on gh-pages](https://heroickatora.github.io/wasm-as-html/examples/yew/todomvc.html).

## How it works

The packer takes two arguments: A webassembly module, and a stage 2 loader
javascript file that will be given control over the page and passed the a
(fake) Promise to a Response that resolves to the web assembly bytes. The
recommended stage 2 is some web-sys implementation that then passes control
directly to a webassembly module. Its `init` and default export take this exact
argument to instantiate the bindings marked as its main function.

The technical inner workings:
* The first section in WASM is a custom one that parses as HTML. The trick take
  from [here](https://fuzzinglabs.com/polyglot-webassembly-module-html-js-wasm/)
  implemented in an program fashion. Note that this `stage0` is not of
  arbitrary size. Otherwise the WebAssembly length encoding of the section's
  length will cause problems as they parse as good characters. This reads the
  first custom section of name `polyglot_stage1`.

* The second section is another custom one, which is our stage1 with arbitrary
  contents. We almost immediately dispatch here from stage0 leaving behind all
  notions of HTML that we started with. We're now preparing the web page for
  the module packed as stage2. That is, loading a new skeleton with expected
  elements that the module might need to bootstrap itself.

  It reads the section title polyglot_stage1_html for contents of a new page,
  as well as the first `polyglot_stage2` for the subsequent module.

WIP: additionally, an auxiliary `.zip` file can be passed. The packer then
ensures that the result is _also_ a valid zip archive with all files intact and
such that they are accessible from the webassembly module as a custom module.
