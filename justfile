alias r := run
alias b := build
alias t := test
alias rr := run-release
alias br := build-release
alias tr := test-release

export RUSTFLAGS := "-Awarnings"

run BIN:
	cargo run --bin {{BIN}}
build BIN:
	cargo build --bin {{BIN}}
run-release BIN:
	cargo run --release --bin {{BIN}}
build-release BIN:
	cargo build --release --bin {{BIN}}
test BIN:
	cargo build --bin {{BIN}}
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
test-release BIN:
	cargo build --release --bin {{BIN}}
	@fish -c "cp_eval_test ./target/debug/{{BIN}} {{BIN}}"
