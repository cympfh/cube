.PHONY: cube wasm pages benchmark test

all: cube wasm pages

cube:
	cargo build
	cargo build --release

wasm:
	wasm-pack build --target web -d pages/pkg

pages: wasm
	make -C pages build
	mkdir -p docs
	rsync -r pages/dist/ docs/

benchmark:
	cargo build --release
	hyperfine -r 10 'cargo run --release -- --roux < tests/random/1'
	hyperfine -r 10 'cargo run --release -- --roux < tests/random/2'
	hyperfine -r 2 'cargo run --release -- --cfop < tests/random/1'
	hyperfine -r 2 'cargo run --release -- --cfop < tests/random/2'

test:
	[ true = $$(cargo run -q --release -- -q -DU < tests/simple/ddu | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q -FURLBD --max-depth 5 < tests/simple/small | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/1 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/2 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/3 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/random/1 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/random/2 | jq .ok) ]
