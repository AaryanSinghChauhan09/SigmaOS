# Device Management

The Sovereign Device Manager handles hardware enumeration, driver binding, and dynamic device changes.

## Device Tree

Devices are structured hierarchically:
- **Root Devices**: Platform devices with no parent.
- **Buses**: PCI and USB controllers act as parents to attached devices.
- **Leaves**: Endpoint devices like keyboards, NVMe drives, and NICs.

## Device Types

Supported hardware types include:
- `BLOCK` (Storage)
- `CHAR` (Serial/Terminal)
- `NET` (Networking)
- `GPU` (Graphics)
- `INPUT` (Keyboard/Mouse)
- `USB` (Controllers)

## Hotplug Subsystem

Devices flagged as `hotpluggable` (like USB and some Block storage) emit events to a kernel-level queue:
- `HOTPLUG_ARRIVAL`
- `HOTPLUG_REMOVAL`

The system processes these events asynchronously to bind or unbind drivers without polling.
