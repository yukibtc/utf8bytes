#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt --all -- --config format_code_in_doc_comments=true

check:
	cargo check --no-default-features
	cargo check --no-default-features --features alloc
	cargo check --no-default-features --features std

clippy:
	cargo clippy --no-default-features -- -D warnings
	cargo clippy --no-default-features --features alloc -- -D warnings
	cargo clippy --no-default-features --features std -- -D warnings

test:
	cargo test --no-default-features
	cargo test --no-default-features --features alloc
	cargo test --no-default-features --features std

precommit: fmt check clippy test
