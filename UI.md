# UI (Zenith UI CSS Engine)

SigmaOS implements the **Zenith UI CSS Engine**, a zero-dependency, native DOM compositing layer designed to definitively surpass the design reputation of **elementary OS**, **Solus**, and **EndeavourOS**.

## Engineering Principles

Located in the /ui/ module of the codebase, the Zenith engine achieves its competitive edge without relying on heavy external dependencies like GTK or Qt:

1. **Sovereign Rendering Pipelines**: Utilizes core CSS custom properties (--zenith-*) to establish a dynamic, glassmorphism-based compositor (.zenith-window).
2. **Native Accessibility Layers**:

* Strict focus-ring contrast (:focus-visible).
* Native media query hooks (@media (prefers-contrast: more)) that automatically drop the background to true black #000000 and shift borders to high-contrast white #ffffff.

1. **Dynamic Customization**: Implements modern typography (Inter), subtle micro-animations (cubic-bezier transitions), and accent glows to deliver a premium, fluid desktop experience.

By isolating the UI rendering pipeline into a sovereign CSS engine, SigmaOS ensures that graphical accessibility and polish are hardcoded at the OS level, neutralizing elementary OS's primary selling point.
