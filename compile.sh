trap "kill 0" SIGINT
# (pushd wasi-loader && watch -n 7 'node --input-type=module < build.js')&>/dev/null &
(pushd stage3/unzip && watch -n 2 'cargo build --release --target=wasm32-wasi')&
export WAH_POLYGLOT_EXPERIMENTAL=y

if false; then
watch ./target/release/wasm-as-html \
	-o out.html \
	--edit \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--trailing-zip data.zip \
	wasi-loader/out.js \
	examples/wasi/wasi-example.wasm
fi

if false; then
watch ./target/release/wasm-as-html \
	-o out.html \
	--edit \
	--add-section wah_polyglot_stage1_html,examples/yew/yew/examples/todomvc/index.html \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--add-section wah_polyglot_wasm_bindgen,examples/yew/yew/target/generated/todomvc.js \
	--trailing-zip data.zip \
	wasi-loader/out.js \
	examples/yew/yew/target/generated/todomvc_bg.wasm
fi

(python -m http.server)&>/dev/null &

if false; then
watch ./target/release/wasm-as-html \
	-o out.html \
	--add-section wah_polyglot_stage1_html,examples/yew/yew/examples/todomvc/index.html \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--add-section wah_polyglot_wasm_bindgen,scene-viewer/scene-viewer.js \
	--trailing-zip /home/andreas/code/projects/rend3/examples/scene-viewer/resources/assets.zip \
	wasi-loader/out.js \
  /home/andreas/code/projects/rend3/target/generated/scene-viewer_bg.wasm
fi

if false; then
watch ./target/release/wasm-as-html \
	-o out.html \
	--add-section wah_polyglot_stage1_html,examples/yew/yew/examples/todomvc/index.html \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--add-section wah_polyglot_wasm_bindgen,scene-viewer/scene-viewer.js \
	--trailing-zip data.zip \
	wasi-loader/out.js \
  /home/andreas/code/projects/rend3/target/generated/scene-viewer_bg.wasm
fi

if true; then
watch ./target/release/wasm-as-html \
	-o out.html \
	--add-section wah_polyglot_stage1_html,scene-viewer/stealth-paint-editor.html \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--add-section wah_polyglot_wasm_bindgen,scene-viewer/stealth-paint-editor.js \
	--trailing-zip data.zip \
	wasi-loader/out.js \
  /home/andreas/code/projects/stealth-paint/target/generated/stealth-paint-editor_bg.wasm
fi
