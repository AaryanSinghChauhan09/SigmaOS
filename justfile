# SigmaOS Justfile task runner

# Default target: list all available commands
default:
    @just --list

# Build all kernel modules and tools workspace
build target="x86_64":
    @echo "Building SigmaOS target: {{target}}..."
    @cmake -B build -DCMAKE_BUILD_TYPE=Release
    @cmake --build build
    @cargo build --manifest-path tools/Cargo.toml --release

# Run unit tests and integration tests
test:
    @echo "Running all tests..."
    @cargo test --manifest-path tools/Cargo.toml
    @cmd /c "npm run test"

# Run the bootable image in QEMU emulator
run headless="":
    @echo "Launching QEMU emulator..."
    @sigma run {{if headless == "true" { "--headless" } else { "" }}}

# Package everything into a bootable ISO image
iso:
    @echo "Building bootable ESP ISO image..."
    @sigma image build --minimal
