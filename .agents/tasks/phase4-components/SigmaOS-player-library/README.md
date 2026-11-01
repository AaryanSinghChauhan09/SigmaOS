# SigmaOS-player-library

SigmaOS-player-library is the media player playlist library for SigmaOS, providing playlist parsing across common formats (M3U, PLS, XSPF, ASX), media metadata extraction via GStreamer discoverer, and streaming URI resolution for HTTP, RTSP, and MMS sources. It serves as the shared media data layer consumed by SigmaOS-player and other components in the SigmaOS media stack.

**Upstream reference projects:** [xplayer-plparser](https://github.com/linuxmint/xplayer-plparser)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-player-library.git
cd SigmaOS-player-library

# Build with Cargo
cargo build --release

# Run tests
cargo test
```

Dependencies: `rust >= 1.75`, `libgstreamer1.0-dev`, `libgstreamer-plugins-base1.0-dev`
