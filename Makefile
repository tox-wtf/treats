all:
	cargo build --release

check:
	cargo hack check --feature-powerset --no-dev-deps
	cargo hack clippy --feature-powerset --no-dev-deps

test:
	cargo hack test --feature-powerset
	cargo test --doc
