# SigmaOS Roadmap: Built-In Screen Recorder
Record the Zenith desktop to AV1 video files natively.
## Goals
- Frame capture from Wayland compositor buffer
- Real-time AV1 encode with low CPU overhead
## Key Milestones
- [ ] Compositor frame export hook
- [ ] Ring-buffer frame queue
- [ ] AV1 encode + MKV container mux