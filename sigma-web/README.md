# SigmaOS Browser Demo (`sigma-web`)

A fully functional browser-based OS simulator with **24 real Web API hardware drivers**.

## Architecture

```
sigma-web/src/
├── App.tsx                    # Root app with OsProvider + DriversProvider
├── lib/utils.ts               # cn() tailwind utility
├── os/
│   ├── OsContext.tsx          # Window manager state (open/close/move/resize)
│   ├── DriverContext.tsx      # 24-driver aggregate context
│   └── drivers/               # 24 individual Web API driver hooks
│       ├── batteryDriver.ts   # Battery Status API
│       ├── networkDriver.ts   # Network Information API
│       ├── geolocationDriver.ts  # Geolocation API
│       ├── motionDriver.ts    # DeviceMotion + DeviceOrientation
│       ├── ambientLightDriver.ts # AmbientLightSensor + prefers-color-scheme
│       ├── filesystemDriver.ts   # File System Access API
│       ├── opfsDriver.ts      # Origin Private File System
│       ├── audioDriver.ts     # Web Audio API + getUserMedia
│       ├── cameraDriver.ts    # getUserMedia (video)
│       ├── screenCaptureDriver.ts # getDisplayMedia
│       ├── gamepadDriver.ts   # Gamepad API
│       ├── speechDriver.ts    # SpeechRecognition + SpeechSynthesis
│       ├── notificationDriver.ts  # Notifications API
│       ├── clipboardDriver.ts # Clipboard API
│       ├── wakeLockDriver.ts  # Screen Wake Lock API
│       ├── multiScreenDriver.ts   # Window Management API
│       ├── usbDriver.ts       # WebUSB API
│       ├── bluetoothDriver.ts # Web Bluetooth API
│       ├── serialDriver.ts    # Web Serial API
│       ├── hidDriver.ts       # WebHID API
│       ├── midiDriver.ts      # Web MIDI API
│       ├── gpuDriver.ts       # WebGPU API
│       ├── shareDriver.ts     # Web Share API
│       └── pointerDriver.ts   # Pointer Events API
├── apps/
│   ├── registry.tsx           # App registry (9 apps)
│   └── devicemanager/
│       └── DeviceManagerApp.tsx  # 24-panel hardware manager
└── screens/
    └── OsRoot.tsx             # Boot → Login → Desktop (windowed)
```

## 24 Drivers

| Driver | Web API | What it does | 
| --- | --- | --- | 
| Battery | Battery Status API | Real battery level, charging state | 
| Network | Network Information API | Online/offline, speed, RTT | 
| Geolocation | Geolocation API | GPS with live tracking | 
| Motion | DeviceMotion/Orientation | Accelerometer, gyroscope | 
| Ambient Light | AmbientLightSensor | Light level, auto dark/light | 
| Filesystem | File System Access API | Browse and read real disk files | 
| OPFS | Origin Private File System | Persistent in-browser storage | 
| Audio | Web Audio API | Microphone, frequency visualizer | 
| Camera | getUserMedia (video) | Webcam with snapshot | 
| Screen Capture | getDisplayMedia | Screen share + recording | 
| Gamepad | Gamepad API | Controller buttons + axes | 
| Speech | SpeechRecognition/Synthesis | Voice input + TTS | 
| Notifications | Notifications API | System push notifications | 
| Clipboard | Clipboard API | Read/write system clipboard | 
| Wake Lock | Screen Wake Lock API | Prevent screen sleep | 
| Multi-Screen | Window Management API | Detect multiple monitors | 
| USB | WebUSB API | Pair and enumerate USB devices | 
| Bluetooth | Web Bluetooth API | GATT device connection | 
| Serial | Web Serial API | Arduino/ESP32 communication | 
| HID | WebHID API | Gamepads, tablets, keyboards | 
| MIDI | Web MIDI API | Music instrument input/output | 
| GPU | WebGPU API | GPU info, features, limits | 
| Share | Web Share API | Native OS share sheet | 
| Pointer | Pointer Events API | Mouse/pen/touch with pressure | 

## Quick Start

```bash
cd sigma-web
npm install
npm run dev
```

Open `http://localhost:5173`, click "Enter Desktop", and double-click **Device Manager** to explore all 24 drivers.
