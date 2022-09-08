.PHONY: benchmark wasm pages

benchmark:
	cargo build --release
	hyperfine -w 1 -r 10 'cargo run --release -- -frudl < tests/cfop/1'
	hyperfine -r 2 'cargo run --release -- -fur < tests/cfop/7'

wasm:
	wasm-pack build --target web -d pages/pkg

pages:
	make -C pages build
	mkdir -p docs
	rsync -r pages/dist/ docs/
