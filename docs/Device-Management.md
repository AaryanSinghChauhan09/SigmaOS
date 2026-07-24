# Device Management in SigmaOS

Device management mirrors the Linux hotplug architecture:

1. **Detection**: Bus drivers (PCI, USB) probe devices and register them via `sigma_udev_register_device`.
2. **Naming & Rules**: The udev daemon matches custom rules (defined in `/etc/udev/rules.d/`) to set node permissions, create symlinks, or run notification scripts.
3. **Hierarchy**: Registered devices populate `/sys/devices/` class trees through `sigma_sysfs`.

See [sigma_udev.rs](file:///C:/Users/Aaryan/.gemini/antigravity-ide/scratch/SigmaOS/kernel/core/sigma_udev.rs) for more details.
