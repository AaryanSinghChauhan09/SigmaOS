# Audio and Graphics

**Capability state: Prototype.** Some audio and raster image library operations exist; no hardware audio, display, compositor, or multimedia application path is supported. Use the [shared status vocabulary](14-Future-Development.md#work-status-vocabulary) for status terms.

## Current capability

### Audio library

`src/audio/editor.rs` provides in-memory floating-point sample operations, including track mixing, cut/paste, basic filters, peak normalization, and fades. `src/audio/audio_codec.rs` decodes integer PCM RIFF/WAVE (8/16/24/32-bit, 1–8 channels) to interleaved signed 16-bit samples and encodes signed 16-bit PCM as RIFF/WAVE. Malformed and unsupported WAV variants are rejected. FLAC, MP3, and Vorbis decoding is unsupported. There is no file adapter, playback/capture integration, or graphical editor; this does not replace Audacity.

- Normalization scales finite samples to a requested peak from 0.0 through 1.0, preserving relative channel balance. Invalid input is rejected.
- Fade-out reaches zero at the final sample, including a one-sample fade.
- Standalone WAV codec tests: `rustc --edition=2021 --test src/audio/audio_codec.rs -o /tmp/sigmaos_audio_codec_tests && /tmp/sigmaos_audio_codec_tests` (15 passed in the recorded run).
- The audio editor Cargo test command is `cargo test --lib audio::editor::tests -- --nocapture`; execution did not complete in the recorded environment.

### Raster library

`src/graphics/paint.rs` provides `RasterLayer::try_new` for fallible allocation and checked pixel counts for blur and PPM/QOI export buffer validation. The compatibility constructor `RasterLayer::new` can panic on invalid or unallocatable dimensions; use `try_new` for untrusted dimensions. Layer-mask and selection constructors still need matching fallible APIs. Graphics Cargo tests were added, but execution did not complete in the recorded environment.

There is no verified DRM/KMS, GPU, Vulkan, Wayland, camera, or compositor integration. Hardware and protocol names in older proposals are not supported device or API claims. No SigmaOS media player or full audio/image editor is available.

## Design references and roadmap

- **Linux and BSD:** use explicit device boundaries and publish a model-by-model hardware support matrix.
- **Linux Mint:** make media setup and error recovery understandable to new users.
- **Omarchy:** make common media and creative workflows discoverable by keyboard.
- **Arch Linux:** document exact supported configurations and keep package recipes inspectable.

1. Validate a file-backed PCM/WAV playback path and clear errors for unsupported formats before building a VLC-inspired player.
2. Add audio import/export, editing history, project saves, and playback/recording integration before claiming an Audacity alternative.
3. Add pixel import/export, compositing, selection/mask operations, undo/redo, and accessibility before claiming a GIMP alternative.
4. Add hardware backends one device at a time; name the tested model, driver path, operations, and limitations.
5. Package and expose apps only after a verified desktop session, transactional package installation, and recovery path exist.

**Completion evidence:** fixture-based media tests, malformed-input and allocation-failure tests, supported-device tests, accessible desktop workflows, and clean-install/recovery validation are recorded for each claimed capability.

Upstream behavior references: [Audacity Normalize manual](https://manual.audacityteam.org/man/normalize.html) and [GIMP layer-mask manual](https://docs.gimp.org/3.0/en/gimp-layer-mask-edit.html). These document the upstream applications only.
