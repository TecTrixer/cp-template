alias r := run
alias b := build
alias t := test
alias rd := run-debug
alias bd := build-debug
alias td := test-debug
alias rr := run-release
alias br := build-release
alias tr := test-release

export RUSTFLAGS := "-Awarnings"

build BIN:
	@cargo build --bin {{BIN}}
run BIN: (build BIN)
	@./target/debug/{{BIN}}
test BIN: (build BIN)
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
build-debug BIN:
	#!/usr/bin/env bash
	RUSTFLAGS="{{RUSTFLAGS}} --cfg DEBUG"
	cargo build --bin {{BIN}}
run-debug BIN: (build-debug BIN)
	@./target/debug/{{BIN}}
test-debug BIN: (build-debug BIN)
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
build-release BIN:
	@cargo build --release --bin {{BIN}}
run-release BIN: (build-release BIN)
	@./target/release/{{BIN}}
test-release BIN: (build-release BIN)
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"

