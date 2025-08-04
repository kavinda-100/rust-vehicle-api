run:
	cargo fmt
	RUSTFLAGS="-A warnings" cargo watch -x run

# Run tests
test:
	RUSTFLAGS="-A warnings" cargo test

# Show help
help:
	@echo "Available targets:"
	@echo "  run          - Run with warnings suppressed"
	@echo "  test         - Run tests"

.PHONY: run test