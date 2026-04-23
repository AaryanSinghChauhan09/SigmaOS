# Sovereign Extensions (s-ext)

SigmaOS achieves the ultimate pinnacle of **ease of use, customisation, and modularisation** through the `s-ext` (Sovereign Extension) API.

Located in `modules/ext/plugins/extension_api.c`.

## The Problem with Linux Customisation
In standard monolithic kernels or even typical microkernels, customizing how the OS behaves (e.g., adding a custom UI overlay, tweaking the scheduler algorithm for a specific game, or automating a response to a USB plug-in) requires editing system config files, recompiling kernel modules, or running heavy user-space daemons that waste battery.

## The SigmaOS Solution
The `s-ext` API allows authorized user-space applications (possessing the `CAP_EXTEND` capability) to dynamically inject hooks directly into the kernel's critical path at runtime—with zero performance penalty.

### Extreme Personalisation & UI/UX
Extensions can bind to `HOOK_UI_RENDER`. After the Zenith Compositor draws the main glassmorphism UI, it hands the framebuffer directly to your extension. You can draw custom widgets, dynamic crosshairs, or visualisers with 0-latency.

### Algorithmic Customisation
Extensions can bind to `HOOK_SCHED_REWARD`. The core Q-Learning AI scheduler will pass its calculated reward to your extension. If you are a power user playing a game, your extension can artificially inflate the reward for the game's PID, forcing the AI to dedicate maximum CPU timeslices to it dynamically.

### Event-Driven Automations
Extensions can bind to `HOOK_AUTOMATION_EVENT`. Whenever the kernel detects hardware changes or network shifts, your extension's callback fires instantly in Ring 0, allowing you to execute custom scripts without relying on heavy poll-based daemons.

This creates an ecosystem where **SigmaOS is infinitely malleable** while remaining cryptographically secure.
