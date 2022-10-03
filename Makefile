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
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/1'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/2'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/3'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/4'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/5'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/6'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/7'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/8'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --roux < tests/random/9'

benchmark-cfop: benchmark-init
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/1'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/2'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/3'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/4'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/5'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/6'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/7'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/8'
	@hyperfine -w 1 -r 8 'cargo run --release -- -q --cfop < tests/random/9'

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
