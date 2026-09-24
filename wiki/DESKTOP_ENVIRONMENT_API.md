# SigmaOS Desktop Environment API Specification

## 1. Overview

The Desktop Environment API defines the system service contracts and IPC interfaces that enable desktop shell components—panels, status bars, application launchers, system trays, notification daemons, and quick-setting popups—to seamlessly integrate with the SigmaOS desktop shell suite (`Zenith Desktop`).

```
+-------------------------------------------------------------------+
|                        Desktop Environment                        |
|  +------------------+  +-------------------+  +-----------------+  |
|  | Panel / Status   |  | App Launcher /    |  | System Tray &   |  |
|  | Bar Component    |  | Search HUD        |  | Quick Settings  |  |
|  +--------+---------+  +---------+---------+  +--------+--------+  |
+-----------|----------------------|-------------------|------------+
            |                      |                   |
+-----------v----------------------v-------------------v------------+
|                    Desktop Environment API Core                   |
|  +------------------+  +-------------------+  +-----------------+  |
|  | Panel protocol   |  | App Index Service |  | StatusNotifier  |  |
|  +------------------+  +-------------------+  +-----------------+  |
+-------------------------------------------------------------------+
```

## 2. Desktop Shell Protocol Protocols

### 2.1 Layer Shell Protocol (`zenith_layer_shell`)
Extends display server surface management to allow shell applications to anchor panels and overlays to screen edges:
- **`Layer::Background`**: Desktop wallpapers and background widgets.
- **`Layer::Bottom`**: Desktop shortcuts and desktop sticky notes.
- **`Layer::Top`**: System status bars, panels, and docks.
- **`Layer::Overlay`**: Application launchers, quick settings, OS Notification popups, and OSD volume bars.

**Anchor Flags**: Top, Bottom, Left, Right edge positioning with pixel margins and keyboard focus modes (`None`, `Exclusive`, `OnDemand`).

### 2.2 System Tray API (`StatusNotifierItem` / `SNI`)
Provides an asynchronous IPC interface for applications to register tray icons and menus:
- **Icon Properties**: `IconName`, `IconPixmap`, `ToolTip`, `Status` (`Passive`, `Active`, `NeedsAttention`).
- **Context Menus**: Standardized menu descriptor trees rendered natively by the system panel shell.

### 2.3 Application Launcher & Indexing API
- **App Desktop Entries**: Parses `.desktop` files conforming to Freedesktop standards (`Name`, `Exec`, `Icon`, `Categories`, `MimeType`, `Keywords`).
- **Search HUD Service**: High-speed indexing daemon allowing instant application launch, file search, and calculation inputs.

### 2.4 Notification Daemon API (`org.freedesktop.Notifications`)
Handles system and application notifications:
- **Methods**: `Notify(app_name, replaces_id, app_icon, summary, body, actions, hints, expire_timeout) -> id`.
- **Notification Actions**: Interactive action buttons (e.g., "Reply", "Dismiss", "Open Folder").
- **Urgency Levels**: Low, Normal, Critical (critical notifications bypass Do-Not-Disturb modes).

## 3. Quick Settings & Control Center API

Integrates core system services with the desktop environment quick settings HUD:
1. **Network Panel API**: Wi-Fi network scanning, SSID connection, VPN status toggle.
2. **Audio & Volume API**: Default audio sink selection, volume levels, mic mute, PipeWire stream routing.
3. **Power & Battery API**: Power profile selection (`Performance`, `Balanced`, `Power Saver`), battery health percentage, remaining charge time.
4. **Bluetooth API**: Device pairing, connection state, battery status for connected peripherals.

## 4. Rust API Usage Example

```rust
use sigmaos_desktop_api::panel::{PanelBuilder, Anchor, Layer};
use sigmaos_desktop_api::notification::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register a top system panel
    let _panel = PanelBuilder::new()
        .set_height(32)
        .set_anchor(Anchor::Top | Anchor::Left | Anchor::Right)
        .set_layer(Layer::Top)
        .set_exclusive_zone(32)
        .build()?;

    // Send a desktop notification
    Notification::new()
        .summary("SigmaOS Update Available")
        .body("Version 1.0.0 is ready for installation.")
        .icon("system-software-update")
        .timeout_ms(5000)
        .show()?;

    Ok(())
}
```
