# SigmaOS-screen-recorder

## Overview
Desktop screen recording tool for SigmaOS.

## Features
- Core functionality for SigmaOS-screen-recorder
- Integration with SigmaOS desktop environment
- Configurable settings and preferences

## Build
```bash
cargo build --release
```

## License
MIT — See LICENSE file for details.

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [omareel (Omarchy)](https://github.com/basecamp/omarchy) | `CaptureRegion`, `RecorderConfig`, `Recorder` state machine with `start`/`stop` transitions |
| [wf-recorder](https://github.com/ammen99/wf-recorder) | Region/display capture model; container + fps configuration |
