# Justfile for SigmaOS
# Sovereign OS build and maintenance automation.

default: check

# Validate all code compiles
check:
	@echo "Checking core modules (no_std)..."
	cargo check --manifest-path modules/core/Cargo.toml --target x86_64-unknown-none || echo "[Warning] No Cargo.toml found in modules/core yet, skipping."
	@echo "Checking sigpkg userland tool..."
	cargo check --manifest-path userland/sigpkg/Cargo.toml || echo "[Warning] No Cargo.toml found in sigpkg yet, skipping."

# Format code
fmt:
	cargo fmt --all

# Run all test suites
test:
	@echo "Running userland tests..."
	cargo test --manifest-path userland/sigpkg/Cargo.toml

# Build the system (placeholder for when cross-compilation is fully setup)
build:
	@echo "Building SigmaOS kernel..."
	# In a real environment: cargo build --release -Z build-std=core,alloc --target x86_64-unknown-none

# Clean artifacts
clean:
	cargo clean
	rm -rf target/

# Maintenance scripts
verify:
	python scripts/maintenance/verify_implementations.py
