# Driver Development Guide

This page documents the SigmaOS Unified Driver API — the common modular interface for all hardware peripherals. It is sourced from the `drivers-dev` branch and applies to any driver targeting the SigmaOS kernel.

---

## Design Principles

- **Zero monolithic dependencies** — driver code must not include `<linux/...>` headers or any GNU userland headers. Use `include/sigma_kernel_types.h` exclusively.
- **Separate binaries** — each driver compiles to its own SDF (Sovereign Driver Format) binary. WiFi and Bluetooth are separate targets; they must not be compiled into the same binary (see `CMakeLists.txt`).
- **O(1) registration and lookup** — drivers register via a lock-free registry; the kernel looks them up in O(1) by device class and MMIO address.
- **Sysctl-tunable** — every meaningful runtime parameter should be registered as a sysctl node.

---

## The `driver_ops` Interface

All drivers implement the same four-hook struct defined in `include/driver_api.h`:

```c
// include/driver_api.h
typedef struct driver_ops {
    /* Called once during kernel driver subsystem init */
    int  (*init)(struct driver_t* drv);

    /* Called to read data from the device into buf */
    int  (*read)(struct driver_t* drv, void* buf, sigma_size_t len, sigma_u64 offset);

    /* Called to write data from buf to the device */
    int  (*write)(struct driver_t* drv, const void* buf, sigma_size_t len, sigma_u64 offset);

    /* Called during shutdown / driver unload */
    void (*shutdown)(struct driver_t* drv);
} driver_ops_t;

typedef struct driver_t {
    char           name[64];        /* e.g. "e1000", "sigma-bt-hci"      */
    driver_ops_t   ops;
    void*          private_data;    /* driver-owned state pointer         */
    sigma_u64      mmio_base;       /* MMIO base address from PCI scan    */
    sigma_u32      irq_line;        /* assigned IRQ from IDT              */
    sigma_sysctl_node_t* sysctl;   /* optional sysctl registration       */
} driver_t;
```

---

## Writing a Minimal Driver

Here is a skeleton for a character device driver (e.g., a GPIO controller):

```cpp
// SPDX-License-Identifier: GPL-2.0-or-later
// kernel/drivers/gpio/sigma_gpio.cpp

#include "driver_api.h"
#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "include/sigma_sysctl.h"

static int  gpio_init(driver_t* drv);
static int  gpio_read(driver_t* drv, void* buf, sigma_size_t len, sigma_u64 off);
static int  gpio_write(driver_t* drv, const void* buf, sigma_size_t len, sigma_u64 off);
static void gpio_shutdown(driver_t* drv);

/* Sysctl: expose debounce interval for runtime tuning */
static int g_debounce_ms = 10;
static sigma_sysctl_node_t g_sysctl_debounce;

static driver_t gpio_driver = {
    .name = "sigma-gpio",
    .ops  = { gpio_init, gpio_read, gpio_write, gpio_shutdown },
};

static int gpio_init(driver_t* drv) {
    /* Map MMIO region from PCI scan result */
    volatile sigma_u32* regs = (volatile sigma_u32*)drv->mmio_base;

    /* Enable controller */
    regs[0] = 0x01;

    /* Register sysctl node */
    sigma_sysctl_register(&g_sysctl_debounce,
                          "drivers.gpio.debounce_ms",
                          SYSCTL_TYPE_INT,
                          &g_debounce_ms, false);

    sigma_log_info("[sigma-gpio] initialised at MMIO 0x%llx\n", drv->mmio_base);
    return 0;
}

static int gpio_read(driver_t* drv, void* buf, sigma_size_t len, sigma_u64 off) {
    volatile sigma_u32* regs = (volatile sigma_u32*)drv->mmio_base;
    if (len < sizeof(sigma_u32)) return -1;
    *((sigma_u32*)buf) = regs[off / 4];
    return (int)sizeof(sigma_u32);
}

static int gpio_write(driver_t* drv, const void* buf, sigma_size_t len, sigma_u64 off) {
    volatile sigma_u32* regs = (volatile sigma_u32*)drv->mmio_base;
    if (len < sizeof(sigma_u32)) return -1;
    regs[off / 4] = *((const sigma_u32*)buf);
    return (int)sizeof(sigma_u32);
}

static void gpio_shutdown(driver_t* drv) {
    volatile sigma_u32* regs = (volatile sigma_u32*)drv->mmio_base;
    regs[0] = 0x00;  /* disable controller */
    sigma_log_info("[sigma-gpio] shutdown\n");
}

/* Called by the kernel driver manager during PCI enumeration */
extern "C" driver_t* sigma_gpio_driver_entry(void) {
    return &gpio_driver;
}
```

---

## PCI Enumeration Integration

Drivers are discovered when the PCI bus scanner (`kernel/drivers/compat/linux_shim.cpp`) finds a matching Vendor ID + Device ID. Register your driver's PCI ID in `kernel/drivers/sigma_driver_registry.c`:

```c
// sigma_driver_registry.c
static const pci_driver_entry_t driver_table[] = {
    { 0x8086, 0x100E, sigma_e1000_driver_entry  },  // Intel e1000
    { 0x1234, 0xABCD, sigma_gpio_driver_entry   },  // Our GPIO controller
    { 0,      0,      NULL                      },  // sentinel
};
```

The PCI scanner calls `driver_entry()` for every matching device found, passing the MMIO base address and assigned IRQ line.

---

## Separate WiFi and Bluetooth Targets

Per the build system requirements (CMakeLists.txt bug fix), each driver must be its own target:

```cmake
# Correct — separate binaries
add_executable(sigma-drv-bt
    kernel/drivers/net/sigma_bluetooth.cpp
    kernel/drivers/net/bt/sigma_bt_hci.cpp
)

add_executable(sigma-drv-wifi
    kernel/drivers/net/sigma_wifi.cpp
    kernel/drivers/net/wifi/sigma_80211.cpp
)

# Wrong — never do this:
# add_executable(sigma-drv-bt ${ALL_NET_SOURCES})  ← includes WiFi sources
```

---

## Testing Your Driver

Add a regression test in `tests/kernel/`:

```cpp
// tests/kernel/drivers/test_gpio.cpp
// SPDX-License-Identifier: GPL-2.0-or-later
#include <cassert>
#include <cstdio>
#include "driver_api.h"

// Stub MMIO region — 16 registers
static sigma_u32 mock_regs[16] = {};

// Override mmio_base to point at mock_regs
int main(void) {
    driver_t* drv = sigma_gpio_driver_entry();
    drv->mmio_base = (sigma_u64)(uintptr_t)mock_regs;

    assert(drv->ops.init(drv) == 0 && "init must succeed");
    assert(mock_regs[0] == 0x01  && "init must enable controller");

    sigma_u32 val = 0xDEAD;
    drv->ops.write(drv, &val, 4, 0);
    sigma_u32 out = 0;
    drv->ops.read(drv, &out, 4, 0);
    assert(out == 0xDEAD && "read must return written value");

    drv->ops.shutdown(drv);
    assert(mock_regs[0] == 0x00 && "shutdown must disable controller");

    printf("test_gpio: PASS\n");
    return 0;
}
```

---

## Driver Status

| Driver | File | Status | 
| --- | --- | --- | 
| VGA text mode | `kernel/drivers/vga/` | ✓ Working | 
| PS/2 Keyboard | `kernel/drivers/input/` | ✓ Working | 
| COM1 Serial | `kernel/core/hal/` | ✓ Working | 
| ATA Disk (PIO) | `kernel/storage/` | ✓ Working | 
| Loopback NIC | `kernel/net/` | ✓ Working | 
| Virtio-net (QEMU) | `kernel/drivers/net/` | ⚠ Partial | 
| e1000 NIC | `kernel/drivers/net/` | ☐ Planned | 
| USB 3.0 xHCI | `kernel/drivers/usb/` | ☐ Planned | 
| NVMe | `kernel/drivers/storage/` | ☐ Planned | 
| Bluetooth HCI | `kernel/drivers/net/bt/` | ⚠ Partial | 
| Wi-Fi 802.11 | `kernel/drivers/net/wifi/` | ⚠ Partial | 
| DRM/KMS shim | `kernel/drivers/gpu/` | ☐ Planned | 

---

*See also: [Kernel Architecture](Kernel) · [HAL](HAL) · [Contributor Roadmap](Contributor-Roadmap)*
