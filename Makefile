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

benchmark: benchmark-roux benchmark-cfop

benchmark-init:
	@cargo build --release

benchmark-roux: benchmark-init
	@hyperfine -r 20 'cargo run --release -- -q --roux < tests/random/1'
	@hyperfine -r 20 'cargo run --release -- -q --roux < tests/random/2'

benchmark-cfop: benchmark-init
	@hyperfine -r 5 'cargo run --release -- -q --cfop < tests/random/1'
	@hyperfine -r 5 'cargo run --release -- -q --cfop < tests/random/2'

test:
	cargo test
	[ true = $$(cargo run -q --release -- -q -DU < tests/simple/ddu | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q -FURLBD < tests/simple/small | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q -RBFD < tests/simple/tiny | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/nop | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/simple/tiny | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/1 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/2 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --roux < tests/random/3 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/nop | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/simple/tiny | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/random/1 | jq .ok) ]
	[ true = $$(cargo run -q --release -- -q --cfop < tests/random/2 | jq .ok) ]
