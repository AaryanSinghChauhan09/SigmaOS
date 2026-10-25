# SigmaOS I/O Callback Policy

## Overview
The Callback Manager (`CallbackManager`) registers, manages, and executes completion callbacks for asynchronous I/O operations.

## Requirements & Enforcements
- **Exact-Once Execution**: Callbacks are invoked exactly once per request completion.
- **Typed Status Reporting**: Returns structured `CallbackStatus` (`Success`, `Timeout`, `HardwareError`, `Canceled`).
- **Multi-Callback Support**: Supports multiple callback handlers per request ID.
- **Resource Reclaim**: Automatically unregisters and frees callback handles upon invocation to prevent leaks.
