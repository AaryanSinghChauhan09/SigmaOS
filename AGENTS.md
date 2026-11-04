# SigmaOS Contributor & Agent Guide

## Engineering Cadence
- Daily: unit testing and clean room development
- Weekly: PR review and subsystem synchronization
- Biweekly: sprint review and benchmark verification
- Monthly: roadmap sync and architecture review
- Quarterly: security review and release planning

## Code Rules
- Clean-room implementations only.
- Strict quality standards.

## Testing Rules
- Every feature must include automated tests.

## Documentation Rules
- Comprehensive docs in docs/ and wiki/.

## Security Rules
- No unvetted dependencies or unsafe code without justification.

## Required Verification Commands
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```
