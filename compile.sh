(pushd wasi-loader && watch -n 7 'node --input-type=module < build.js')&

watch ./target/release/wasm-as-html \
	-o out.html \
	--edit \
	wasi-loader/out.js \
	examples/wasi/wasi-example.wasm \
	--trailing-zip data.zip
