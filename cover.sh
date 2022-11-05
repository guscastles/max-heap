#!/bin/bash

CARGO_INCREMENTAL=0 \
	RUSTFLAGS='-Cinstrument-coverage' \
	LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' \
	cargo test

~/.cargo/bin/grcov . \
	--binary-path ./target/debug/deps/ \
	-s . \
	-t html \
	--branch \
	--ignore-not-existing \
	--ignore '../*' \
	--ignore "/*" \
	--ignore "src/lib.rs" \
	-o target/coverage/html
