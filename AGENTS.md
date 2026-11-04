# SigmaOS Governance and Guidelines

## Engineering Cadence
- Daily
- Weekly
- Biweekly
- Monthly
- Quarterly

## Rules
### Code Rules
- Follow safe Rust practices.
- cargo fmt --check
- cargo clippy --all-targets -- -D warnings
- cargo check --lib

### Testing Rules
- cargo test --lib
- ./run_sigma_tests.sh

### Documentation Rules
- Keep ADRs and design docs current.

### Security Rules
- No memory unsafety, verify permissions.
