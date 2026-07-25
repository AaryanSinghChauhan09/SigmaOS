# Multi-Monitor KMS Management

SigmaOS's display stack supports multiple concurrent monitor outputs using Kernel Mode Setting (KMS) interfaces.

## 1. Monitor Output Discovery
- Detects DisplayPort, HDMI, eDP, and other standard physical connectors.
- Automatically queries EDID information to fetch preferred resolutions and refresh rates.

## 2. CRTC Allocations
- Dynamically assigns active CRTC controllers to active output connectors.
- Supports cloning and extended desktop layout configurations.
