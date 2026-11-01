# SigmaOS-upgrade-legacy

Legacy system upgrade support tool for SigmaOS. Enables upgrade paths from older or unsupported SigmaOS versions, handling compatibility and migration tasks.

## Upstream Inspiration

- [mintupgrade](https://github.com/linuxmint/mintupgrade) (legacy paths)

## Build

```bash
cargo build --release
```

## Principles

- **DRY**: shared migration logic lives in `migration` module; compatibility checks in `compat`
- **Modularity**: each concern is its own module
- **FOSS**: MIT licensed
- **KISS**: minimal surface area; upgrade logic is explicit and auditable
- **Discoverability**: `init()` entry point wires modules for consumers

## License

MIT 2025 SigmaOS Contributors
