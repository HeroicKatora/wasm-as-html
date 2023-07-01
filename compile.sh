(pushd wasi-loader && watch -n 7 'node --input-type=module < build.js')&>/dev/null &
# (pushd stage3/unzip && watch -n 7 'cargo build --release --target=wasm32-wasi')&
export WAH_POLYGLOT_EXPERIMENTAL=y

watch ./target/release/wasm-as-html \
	-o out.html \
	--edit \
	--add-section wah_polyglot_stage3,target/wasm32-wasi/release/unzip.wasm \
	--trailing-zip data.zip \
	wasi-loader/out.js \
	examples/wasi/wasi-example.wasm \
