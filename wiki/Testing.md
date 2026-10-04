# Testing

## Status (Oct 2026)
- `cargo test --lib`: 6346+ tests passing, 0 failing, ~67 ignored.
- Modules behind `#[cfg(test_disabled)]` are known-broken test suites kept compilable-only.
- AES-GCM vault encryption is a stub (`CryptoUnavailable`) until a pure-Rust provider lands.

## How to run
```
cargo test --lib                 # full unit suite
cargo test --lib <name>          # single module
cargo check                      # fast compile check
```

## AI Agent Maintenance
- Never blanket `#[cfg(test_disabled)]` a module; instead gate only the specific broken `#[test]` fn with `#[ignore]`.
- Re-enable ignored tests by fixing the panic message shown by `cargo test --lib -- --ignored`.
