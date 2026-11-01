# SigmaOS-network

Networking stack: TCP/IP, UDP, DNS, TLS, and mDNS discovery. Packet filtering and QoS support.

## Component Purpose

SigmaOS-network provides:

- Core component functionality
- Integration with SigmaOS ecosystem

## Architecture

Organized into subsystems:

src/tcp/, src/udp/, src/dns/, src/tls/, src/netfilter/

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
