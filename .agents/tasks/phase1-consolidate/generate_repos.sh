#!/bin/bash
set -e

BASE_DIR="/home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase1-consolidate"
LICENSE_TEMPLATE="$BASE_DIR/LICENSE-TEMPLATE"

# Repository definitions
declare -A REPOS=(
  [SigmaOS-filesystem]="Virtual filesystem layer with ext4, tmpfs, and devfs drivers. Provides efficient, zero-copy paths and journaling support."
  [SigmaOS-network]="Networking stack: TCP/IP, UDP, DNS, TLS, and mDNS discovery. Packet filtering and QoS support."
  [SigmaOS-security]="Privilege separation, LSM hooks, capability-based security, and secure defaults."
  [SigmaOS-system]="Init system, service manager, device discovery, and system orchestration."
  [SigmaOS-desktop]="Wayland compositor shell and desktop primitives (panels, menus, window management)."
  [SigmaOS-packages]="Package manager core: dependency resolution, installation, verification, and rollback."
)

# Additional info for each repo
declare -A SUBSYSTEMS=(
  [SigmaOS-filesystem]="src/vfs/, src/drivers/, src/journal/, src/cache/"
  [SigmaOS-network]="src/tcp/, src/udp/, src/dns/, src/tls/, src/netfilter/"
  [SigmaOS-security]="src/privilege/, src/lsm/, src/capabilities/, src/audit/"
  [SigmaOS-system]="src/init/, src/services/, src/discovery/, src/orchestration/"
  [SigmaOS-desktop]="src/wayland/, src/input/, src/output/, src/shell/"
  [SigmaOS-packages]="src/resolver/, src/installer/, src/verify/, src/rollback/"
)

create_repo() {
  local repo_name=$1
  local description=$2
  local subsystems=$3
  local repo_dir="$BASE_DIR/$repo_name"
  
  echo "Creating $repo_name..."
  
  mkdir -p "$repo_dir/.github"
  
  # README.md
  cat > "$repo_dir/README.md" << 'EOF'
# {REPO_NAME}

{DESCRIPTION}

## Component Purpose

{REPO_NAME} provides:

{FEATURES}

## Architecture

Organized into subsystems:

{SUBSYSTEMS}

## Dependencies

**Internal (SigmaOS):**
- SigmaOS-kernel — Core system interface

**External:**
- thiserror — Error handling
- log — Structured logging
- parking_lot — Synchronization primitives

## Build Instructions

### Prerequisites

- Rust 1.70+
- Target: x86_64-unknown-linux-gnu or aarch64-unknown-linux-gnu

### Building

```bash
# Release build
cargo build --release

# With all features
cargo build --release --all-features

# Testing
cargo build --tests
```

## Testing Instructions

### Unit Tests

```bash
# Run all tests
cargo test --lib

# Run with output
cargo test --lib -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'
```

## Contributing

See [CONTRIBUTE.md](./CONTRIBUTE.md) for guidelines.

## License

MIT License. See [LICENSE](./LICENSE) for details.

## Related Documentation

- [SigmaOS Documentation](https://github.com/AaryanSinghChauhan09/SigmaOS-docs)
- [Component Registry](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/tree/main/docs/registry)
EOF

  # Replace placeholders
  sed -i "s|{REPO_NAME}|$repo_name|g" "$repo_dir/README.md"
  sed -i "s|{DESCRIPTION}|$description|g" "$repo_dir/README.md"
  sed -i "s|{SUBSYSTEMS}|$subsystems|g" "$repo_dir/README.md"
  sed -i "s|{FEATURES}|- Core component functionality\n- Integration with SigmaOS ecosystem|g" "$repo_dir/README.md"

  # CONTRIBUTE.md (generic version)
  cat > "$repo_dir/CONTRIBUTE.md" << 'EOF'
# Contributing to {REPO_NAME}

Thank you for contributing to {REPO_NAME}!

## Code of Conduct

By participating, you agree to uphold the [SigmaOS Code of Conduct](https://github.com/AaryanSinghChauhan09/SigmaOS-docs/blob/main/docs/governance/CODE_OF_CONDUCT.md).

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/your-username/{REPO_NAME}.git
cd {REPO_NAME}
```

### 2. Create a Feature Branch

```bash
git checkout -b feature/my-feature
```

### 3. Make Your Changes

- Follow SOLID principles
- Write tests for new code
- Update documentation

### 4. Run Tests Locally

```bash
cargo test --lib
cargo fmt --check
cargo clippy -- -D warnings
```

### 5. Commit and Push

```bash
git add .
git commit -m "feat: description of changes"
git push origin feature/my-feature
```

### 6. Create a Pull Request

Open a PR with:
- Clear title and description
- Reference to related issues
- Test results

## Code Review

Reviews will focus on:
- Code quality and SOLID principles
- Test coverage
- Documentation accuracy
- Performance implications
- Security considerations

## Questions?

Refer to [SigmaOS Documentation](https://github.com/AaryanSinghChauhan09/SigmaOS-docs) or reach out to maintainers.
EOF

  sed -i "s|{REPO_NAME}|$repo_name|g" "$repo_dir/CONTRIBUTE.md"

  # Cargo.toml
  cat > "$repo_dir/Cargo.toml" << 'EOF'
[package]
name = "{CARGO_NAME}"
version = "0.1.0"
edition = "2021"
authors = ["Aaryan Singh Chauhan <aaryan@sigmaos.dev>"]
license = "MIT"
description = "{DESCRIPTION}"
repository = "https://github.com/AaryanSinghChauhan09/{REPO_NAME}"
keywords = ["sigmaos", "system"]
categories = ["os"]

[dependencies]
thiserror = "1.0"
log = "0.4"
parking_lot = "0.12"

[dev-dependencies]
criterion = "0.5"

[[test]]
name = "integration"
path = "tests/integration.rs"
EOF

  local cargo_name=$(echo "$repo_name" | tr '-' '_')
  sed -i "s|{CARGO_NAME}|$cargo_name|g" "$repo_dir/Cargo.toml"
  sed -i "s|{DESCRIPTION}|$description|g" "$repo_dir/Cargo.toml"
  sed -i "s|{REPO_NAME}|$repo_name|g" "$repo_dir/Cargo.toml"

  # .gitignore (Rust standard)
  cat > "$repo_dir/.gitignore" << 'EOF'
# Rust
/target/
**/*.rs.bk
Cargo.lock
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store
*.o
*.a
*.so
*.dylib
*.exe
*.elf
.tarpaulin-report.html
cobertura.xml
/fuzz/target/
/fuzz/artifacts/
/docs/generated/
Thumbs.db
*.tmp
*.bak
EOF

  # LICENSE
  cp "$LICENSE_TEMPLATE" "$repo_dir/LICENSE"

  # CI workflow
  cat > "$repo_dir/.github/workflows-component-ci.yml" << 'EOF'
name: Component CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 2 * * *'

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo check --verbose --all-features

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --verbose --lib
      - run: cargo test --verbose --test '*'

  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --all-targets --all-features -- -D warnings

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: cargo tarpaulin --out Xml --exclude-files tests/* --timeout 300
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml
          fail_ci_if_error: false

  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: rustsec/audit-check-action@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
EOF

  echo "✓ Created $repo_name"
}

# Create all remaining repos
for repo_name in "${!REPOS[@]}"; do
  [ "$repo_name" = "SigmaOS-kernel" ] && continue  # Already created
  description="${REPOS[$repo_name]}"
  subsystems="${SUBSYSTEMS[$repo_name]}"
  create_repo "$repo_name" "$description" "$subsystems"
done

echo ""
echo "All repositories created successfully!"
