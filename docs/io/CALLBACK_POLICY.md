# SigmaOS I/O Callback Policy

## 1. Overview
The Callback Manager subsystem (`src/io/callback.rs`) manages completion notifications for scheduled I/O operations.

## 2. Key Invariants and Rules

### 2.1 Single-Invocation Guarantee
- Registered callbacks for a request ID are executed exactly once when `invoke_callbacks` is called.
- After invocation, all handles associated with the request ID are removed from the registration map, preventing double-invocation races.

### 2.2 Callback Status
Callbacks receive a `CallbackStatus` outcome enum:
- `Success`: Operation completed normally.
- `Timeout`: Device operation timed out before hardware confirmation.
- `HardwareError`: Controller or media failure reported by device.
- `Canceled`: Operation canceled due to device hotplug or system shutdown.
