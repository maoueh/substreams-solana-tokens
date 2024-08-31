ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) tokens/substreams.yaml db_out -s 286135778 -t +10

.PHONY: protogen
protogen:
	cd tokens
	substreams protogen ./substreams.yaml --exclude-paths="sf/solana/type,sf/substreams,google"

.PHONY: pack
pack:
	substreams pack ./substreams.yaml