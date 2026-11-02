# SigmaOS-audio-tuner

Audio equalizer and tuner application providing per-application audio effects, filtering, and hardware tuning. Builds on the EasyEffects pipeline model with SigmaOS-native integration.

**Upstream inspiration:** https://github.com/wwmm/easyeffects

**Dependencies:** `pipewire`, `gstreamer`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-audio-tuner /usr/bin/sigma-audio-tuner
```
