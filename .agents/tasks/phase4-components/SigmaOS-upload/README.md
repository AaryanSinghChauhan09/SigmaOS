# SigmaOS-upload

File upload utility for SigmaOS. Upload files to remote servers via FTP, SFTP, or HTTP with progress tracking. Inspired by mintupload.

## Upstream Inspiration

- [mintupload](https://github.com/linuxmint/mintupload)

## Build

```bash
cargo build --release
```

## Principles

- **DRY**: protocol adapters share a common upload trait; progress logic is centralised
- **Modularity**: `upload` handles transfer protocols; `progress` handles reporting
- **FOSS**: MIT licensed
- **KISS**: single entry point, minimal config surface
- **Discoverability**: `init()` registers available upload backends

## License

MIT 2025 SigmaOS Contributors
