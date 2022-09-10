.PHONY: benchmark wasm pages

benchmark:
	cargo build --release
	hyperfine -r 10 'cargo run --release -- --roux < tests/random/1'
	hyperfine -r 10 'cargo run --release -- --roux < tests/random/2'
	hyperfine -r 2 'cargo run --release -- --cfop < tests/random/1'
	hyperfine -r 2 'cargo run --release -- --cfop < tests/random/2'

wasm:
	wasm-pack build --target web -d pages/pkg

pages:
	make -C pages build
	mkdir -p docs
	rsync -r pages/dist/ docs/
