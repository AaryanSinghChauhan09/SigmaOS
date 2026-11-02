# SigmaOS-calculator

Calculator application supporting basic and scientific calculations with a GTK frontend and a headless computation engine. Inspired by GNOME Calculator.

**Upstream inspiration:** https://github.com/linuxmint/gnome-calculator

**Dependencies:** `gtk4`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-calculator /usr/bin/sigma-calculator
```
