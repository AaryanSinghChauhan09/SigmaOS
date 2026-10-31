# Palette's Persistent Architectural Journal 🎨

## Rules for this Journal:
- This journal is NOT a routine changelog.
- ONLY record CRITICAL UX, accessibility, and design system learnings.
- Do NOT journal routine work like adding standard labels without architectural impact.

---

## 2026-10-31 - Non-Blocking Asynchronous IPC for Desktop Shell Feedback
**Learning:** Shell status plugins and notification daemons that synchronously queried systemd/sigctl blocked the Wayland compositor event loop on slow I/O, creating visible frame jitter and degraded input responsiveness.
**Action:** Decouple all desktop widget queries through asynchronous event channels or lock-free ring-buffer reads, ensuring that desktop UI components render from memory caches and never block the main presentation thread.
