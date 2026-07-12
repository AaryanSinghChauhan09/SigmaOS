# SigmaOS Browser Demo

SigmaOS ships a complete browser-based OS simulator built in React/TypeScript. It runs entirely in the browser with no server — just open it and you have a working desktop OS with real hardware access via 24 Web API drivers.

## What It Is

`sigma-web/` is a windowed desktop environment that runs in any modern browser (Chrome/Edge recommended for full driver support). It demonstrates the SigmaOS architecture — boot sequence, login, windowed desktop, app registry, and hardware abstraction layer — all running as a Progressive Web App.

## Quick Start

```bash
cd sigma-web
npm install
npm run dev

# → http://localhost:5173

```

1. Boot animation → 2 seconds

2. Login screen → click "Enter Desktop"

3. Desktop with dock → double-click **Device Manager** to see all 24 drivers

---

## Architecture

```
sigma-web/src/
├── App.tsx                       # Root — OsProvider + DriversProvider

├── os/
│   ├── OsContext.tsx             # Window manager state

│   │                             # (open/close/minimize/maximize/drag/resize)

│   ├── DriverContext.tsx         # Aggregates all 24 driver hooks

│   └── drivers/                  # 24 individual Web API hooks

├── apps/
│   ├── registry.tsx              # App definitions (id, name, icon, component)

│   └── devicemanager/
│       └── DeviceManagerApp.tsx  # 24-panel hardware explorer

└── screens/
    └── OsRoot.tsx                # Boot → Login → Desktop with windowed apps

```

---

## 24 Hardware Drivers

Each driver is a React hook that wraps a real Web API. All driver state is live — data updates in real time.

### Sensors

| Driver | Web API | What you see |
|---|---|---|
| **Battery** | Battery Status API | Live % level, charging state, time remaining |
| **Network** | Network Information API | Online/offline, connection type, RTT, speed |
| **Geolocation** | Geolocation API | GPS coordinates with live tracking |
| **Motion** | DeviceMotion + DeviceOrientation | Accelerometer (x/y/z) + gyroscope angles |
| **Ambient Light** | AmbientLightSensor + media query | Light/dark environment, lux value |

### Storage

| Driver | Web API | What you can do |
|---|---|---|
| **Filesystem** | File System Access API | Browse your real disk, open/save files |
| **OPFS** | Origin Private File System | Persistent in-browser virtual disk (read/write/delete) |

### Media

| Driver | Web API | What you see |
|---|---|---|
| **Audio** | Web Audio API + getUserMedia | Microphone with live frequency spectrum visualizer |
| **Camera** | getUserMedia (video) | Live webcam feed with snapshot capture |
| **Screen Capture** | getDisplayMedia | Screen sharing + WebM recording + download |

### Input

| Driver | Web API | What you see |
|---|---|---|
| **Gamepad** | Gamepad API | Live button states + analog axis bars |
| **Speech** | SpeechRecognition + Synthesis | Voice input transcript + text-to-speech |
| **Pointer** | Pointer Events API | Mouse/pen/touch with pressure, tilt, twist |

### System

| Driver | Web API | What you can do |
|---|---|---|
| **Notifications** | Notifications API | Request permission + send system notifications |
| **Clipboard** | Clipboard API | Write to and read from system clipboard |
| **Wake Lock** | Screen Wake Lock API | Prevent screen sleep |
| **Multi-Screen** | Window Management API | Detect monitors, resolution, DPR |

### Hardware Peripherals

| Driver | Web API | What you see |
|---|---|---|
| **USB** | WebUSB API | Pair USB devices, vendor/product IDs |
| **Bluetooth** | Web Bluetooth API | GATT device scan, connect, disconnect |
| **Serial** | Web Serial API | Arduino/ESP32 — send commands, read output |
| **HID** | WebHID API | Gamepads, graphics tablets, specialty keyboards |
| **MIDI** | Web MIDI API | Instrument inputs, play notes on outputs |
| **GPU** | WebGPU API | Adapter info, vendor, supported features, limits |
| **Share** | Web Share API | Native OS share sheet |

---

## Browser Support

| Feature | Chrome/Edge | Firefox | Safari |
|---|---|---|---|
| Battery | ✅ (Chrome-only) | ❌ | ❌ |
| Geolocation | ✅ | ✅ | ✅ |
| WebUSB | ✅ | ❌ | ❌ |
| Web Bluetooth | ✅ | ❌ | ❌ |
| Web Serial | ✅ | ❌ | ❌ |
| WebHID | ✅ | ❌ | ❌ |
| WebGPU | ✅ (Chrome 113+) | Partial | Partial |
| File System Access | ✅ | Partial | ✅ |
| Screen Capture | ✅ | ✅ | ✅ |
| All other drivers | ✅ | ✅ | ✅ |

**Recommendation:** Use Chrome or Edge for full 24-driver support. The Driver Manager shows each driver's real-time status (Active / Available / Unsupported / Denied).

---

## Why This Matters

This browser demo serves three purposes:

1. **Proof of concept** — shows SigmaOS architecture working before native hardware boot exists

2. **Driver showcase** — demonstrates the 24-driver hardware abstraction layer that will map to native kernel drivers

3. **Demo for users** — anyone can try SigmaOS immediately without downloading anything

The mapping from browser drivers to native kernel drivers:

| Browser (sigma-web) | Native target (Phase 2) |
|---|---|
| Battery Status API | ACPI battery HAL driver |
| Network Information API | sigma-netd (DHCP/WiFi/Ethernet) |
| getUserMedia (camera) | V4L2 camera SDF driver |
| WebUSB | USB host controller SDF driver |
| Web Bluetooth | BlueZ userspace SDF driver |
| Web Serial | UART/CDC-ACM SDF driver |
| WebGPU | sigma-drm-* GPU SDF drivers |
| OPFS | SigmaFS / btrfs VFS layer |

---

*See also: [Development Roadmap](Development-Roadmap) · [Gap Analysis](Gap-Analysis) · [SigmaOS Vision for India](SigmaOS-Vision-India)*
