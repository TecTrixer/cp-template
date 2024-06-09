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

run BIN:
	cargo run --bin {{BIN}}
build BIN:
	cargo build --bin {{BIN}}
test BIN:
	cargo build --bin {{BIN}}
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
run-debug BIN:
	cargo run --bin {{BIN}} --features DEBUG
build-debug BIN:
	cargo build --bin {{BIN}} --features DEBUG
test-debug BIN:
	cargo build --bin {{BIN}} --features DEBUG
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
run-release BIN:
	cargo run --release --bin {{BIN}}
build-release BIN:
	cargo build --release --bin {{BIN}}
test-release BIN:
	cargo build --release --bin {{BIN}}
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"

