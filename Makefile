.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy