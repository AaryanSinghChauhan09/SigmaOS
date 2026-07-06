# SigmaOS Roadmap: Hardware-Accelerated Video Codec
AV1 and H.264 encoding/decoding with GPU acceleration.
## Goals
- AV1 decode via dav1d-equivalent Rust port
- H.264 encode using VAAPI hardware backend
## Key Milestones
- [ ] AV1 bitstream parser
- [ ] VAAPI surface allocation in HAL
- [ ] Zenith media player widget