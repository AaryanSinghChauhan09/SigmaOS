# SigmaOS-video-cut

Video editing and cutting tool for SigmaOS. Trim, cut, and merge video files with a simple timeline interface. Inspired by omacut.

## Upstream Inspiration

- [omacut](https://github.com/basecamp/omarchy) (video cut)
- [Kdenlive](https://kdenlive.org/)

## Build

```bash
cargo build --release
```

## Principles

- **DRY**: shared clip manipulation logic lives in `timeline`; rendering/export in `export`
- **Modularity**: timeline and export concerns are fully separated
- **FOSS**: MIT licensed
- **KISS**: minimal UI — trim, cut, merge; no feature creep
- **Discoverability**: `init()` wires timeline and export for consumers

## License

MIT 2025 SigmaOS Contributors
