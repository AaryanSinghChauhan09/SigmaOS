# SigmaOS Gaming Spin — Sovereign Gaming Edition

The **SigmaOS Gaming** spin is SigmaOS's answer to SteamOS — a performance-first, gaming-optimized environment with Proton compatibility, Vulkan-native rendering, and sovereign gamepad support.

---

## 🎮 Core Gaming Stack

| Component | Purpose |
|-----------|---------|
| Steam (Flatpak) | Game library & Proton compatibility |
| Proton-GE | Community-enhanced Wine/Proton layer |
| Lutris | Multi-source game launcher (GOG, Epic, itch.io) |
| Wine / Wine-Staging | Windows game compatibility |
| Heroic Games Launcher | Epic & GOG native client |

## 🖥 Graphics Stack

- **Vulkan / Mesa** — primary rendering API (AMD, Intel open-source drivers)
- **DXVK** — Direct3D 11/12 → Vulkan translation
- **VKD3D-Proton** — DirectX 12 → Vulkan
- **GameMode** — CPU/GPU governor tuning on launch
- **MangoHUD** — real-time overlay: FPS, CPU, GPU, VRAM usage
- **Gamescope** — Wayland micro-compositor for upscaling (like SteamOS)

## 🕹 Gamepad & Controller Support

| Controller | Driver Status |
|-----------|--------------|
| Xbox One/Series | xpadneo kernel module |
| PlayStation 4/5 | hid-playstation kernel module |
| Nintendo Switch Pro | hid-nintendo module |
| Generic USB HID | usbhid (built-in) |

## 🔊 Low-Latency Audio

- **PipeWire** with `pipewire-pulse` — zero-latency game audio
- JACK routing for streaming setups

## 📡 Streaming & Social

- **OBS Studio** — stream to Twitch/YouTube with GPU encoding (NVENC/VAAPI)
- **Discord** (Flatpak) — voice & overlay support
- **Sunshine** — self-hosted game streaming server (Moonlight client compatible)

## ⚡ Performance Tuning

```bash
# Enable GameMode
gamemoded -r

# Apply Proton custom flags
PROTON_USE_WINED3D=0 DXVK_ASYNC=1 %command%

# MangoHUD overlay
MANGOHUD=1 %command%
```

## 🚀 Installation

```bash
sigma-spin install gaming
```

## 📚 See Also

- [Hardware Drivers](Hardware-Drivers.md)
- [Zenith GPU Stack](Hardware-Abstraction-Layer.md)
