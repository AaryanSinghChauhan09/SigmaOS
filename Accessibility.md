# Accessibility & Assistive Standards

This specification details the user-interface contrast metrics, hardware magnification modes, and text-to-speech systems that ensure a sovereign experience for users with differing accessibility needs.

---

## 👁️ Visual Accessibility

### 1. High-Contrast Rendering Mode
Zenith enforces WCAG AAA standard palettes:
- **Default Contrast Ratio**: Min **7.0:1** for normal text.
- **Toggle Mode**: Increases text weights and swaps borders to high-contrast monochrome surfaces (`#000000` / `#FFFFFF`).

### 2. Magnification Engine
The system magnifier (`sigma_magnifier.rs`) utilizes direct framebuffer crops:
- **Scale Range**: **2x to 16x** zoom.
- **Pacing**: Follows focus cursor position with smooth viewport transitions, running at display refresh rates.

---

## 🗣️ Auditory Accessibility

### 1. Screen Reader Text-to-Speech (`sigma_screen_reader.rs`)
- Emits voice output describing the currently active window, selected UI element, and text inputs.
- Synthesizes speech from ARIA labels and title descriptors.
- Supports customizable voice speed, pitch, and localization.

### 2. Visual Alerts
System warnings (bell, error tones) are mapped to desktop indicators:
- Flash screen borders.
- Display transient on-screen notifications.
