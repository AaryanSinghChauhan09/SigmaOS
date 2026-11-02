# SigmaOS-panel

SigmaOS-panel is the desktop taskbar and panel for SigmaOS, providing a fully customisable system panel with a modular applet system, window list, workspace switcher, clock, and system tray. It is built in Rust for performance and memory safety, and supports drag-and-drop applet arrangement, multi-monitor setups, and deep integration with the SigmaOS compositor and notification system.

**Upstream reference projects:** [xfce-panel](https://gitlab.xfce.org/xfce/xfce4-panel)

## Build Instructions

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS-panel.git
cd SigmaOS-panel

# Build with Cargo
cargo build --release

# Run the panel
./target/release/sigma-panel
```

Dependencies: `rust >= 1.75`, `libgtk-4-dev`, `libwayland-dev`
