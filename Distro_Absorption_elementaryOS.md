# Distro Absorption: elementary OS

> **Status**: 📋 Planned | **Source Paradigm**: elementary OS | **Target Shard**: `SigmaOS Zenith Design Language`

---

## 1. Executive Summary

elementary OS stands out in the Linux ecosystem for its ruthless commitment to design consistency, typography, and human interface guidelines (HIG). Through its Pantheon desktop and Granite toolkit, it forces third-party apps to adhere to a cohesive visual standard.

SigmaOS absorbs the **Strict HIG Enforcement** and **AppCenter Monetization Model**, integrating them into the Zenith compositor and the `sigma-pkg` ecosystem to ensure a premium, unified aesthetic.

---

## 2. Key Features to Absorb

### 2.1 Enforced UI Consistency

In standard Linux, applications look completely different depending on whether they use GTK, Qt, or Electron. SigmaOS's Zenith compositor enforces a unified stylesheet at the Wayland protocol layer. If an application utilizes the `sigma-ui` toolkit, it automatically inherits system-wide dark mode, accent colors, and typography. 

For legacy applications (X11/Wayland), Zenith applies specialized shaders and window decorations to force visual compliance.

### 2.2 Curated AppStore (Sigma Hub)

Like the elementary AppCenter, the Sigma Hub is a curated repository of sandboxed applications. 

- **Pay-What-You-Can Model**: Developers can list applications with a suggested price to fund open-source development.
- **Strict Sandboxing**: Every app in the Sigma Hub must be containerized (via `sigma-sandbox`) with explicitly declared permissions (e.g., Network, Webcam, Filesystem), visually presented to the user prior to installation.

---

## 3. References & Standards

- elementary OS — `elementary.io`
- Granite Toolkit & Pantheon
