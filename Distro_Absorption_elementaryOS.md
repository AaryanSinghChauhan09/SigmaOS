# Distro Absorption: elementary OS — Human Interface Guidelines

> **Status**: 📋 Planned | **Source Paradigm**: elementary OS | **Target Shard**: `SigmaOS Zenith UI & AppCenter`

---

## 1. Executive Summary

elementary OS is renowned for its **Pantheon desktop environment**, strict **Human Interface Guidelines (HIG)**, and its curated, pay-what-you-want **AppCenter**. It provides a highly polished, macOS-like experience built on top of Ubuntu.

SigmaOS absorbs elementary's **strict HIG enforcement for first-party apps** and **curated, sandboxed app distribution model** into the Zenith Desktop ecosystem.

---

## 2. Key Features to Absorb

### 2.1 Zenith Human Interface Guidelines

Like elementary's HIG, Zenith Desktop enforces a strict, unified design language for all first-party applications. Applications are required to use the Zenith UI Toolkit, ensuring consistent padding, typography, dark mode support, and keyboard navigation.

```markdown
# Zenith HIG Excerpt:
- Every app MUST support a toggleable pure-black dark mode.
- Custom window titlebars (Client-Side Decorations) are standard; the titlebar MUST contain primary navigation actions.
- Minimum tap target size for touch/mouse is 44x44px.
```

### 2.2 Curated AppCenter

The SigmaOS `sigma-appcenter` is a curated software boutique. Applications submitted to the AppCenter must:
1. Be fully sandboxed using SigmaOS capabilities (no ambient filesystem access).
2. Adhere to the Zenith HIG.
3. Integrate with the Zenith Desktop (notifications, search).

### 2.3 Focus-First Notifications

Notifications in Zenith Desktop are non-intrusive. While in "Focus Mode" (inspired by elementary's Do Not Disturb), only critical system alerts bypass the filter; all other notifications are silently routed to the Notification Center.

---

## 3. References & Standards

- elementary OS — `elementary.io`
- elementary HIG — `docs.elementary.io/hig`
