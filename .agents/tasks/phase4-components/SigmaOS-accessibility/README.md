# SigmaOS-accessibility

Provides the accessibility layer for SigmaOS, exposing AT-SPI2 interfaces so assistive technologies (screen readers, magnifiers, switch access) can interact with all desktop UI components. Built on top of the GNOME Accessibility Toolkit infrastructure with SigmaOS-specific extensions.

**Upstream inspiration:** https://gitlab.gnome.org/GNOME/at-spi2-core

**Dependencies:** `at-spi2-core`, `dbus`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-accessibility /usr/lib/sigma/accessibility
```
