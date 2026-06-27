// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_driver_framework.h — Sigma Driver Framework (SDF)
 *
 * Combines Linux device model (probe/remove lifecycle + sysfs),
 * Windows WDF (IRP async I/O), and macOS IOKit (typed driver families).
 * Adds SigmaOS-unique: Dilithium3 signature verification at load time.
 *
 * Layer overview:
 *   1. SigmaDevice + SigmaDriver — core device/driver structs (ALL drivers)
 *   2. Driver families — typed base classes (Net, Block, Audio, GPU, Input)
 *   3. SigmaIRP — async I/O request packets (like Windows IRP)
 *   4. Bus drivers — PCI, USB, I2C, Platform
 *   5. Userspace drivers — via sigma_driver_sandbox.cpp
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Device classes ──────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_DEV_NET      = 0,   /* ethernet, wifi, bluetooth              */
    SIGMA_DEV_BLOCK    = 1,   /* NVMe, SATA, USB mass storage           */
    SIGMA_DEV_AUDIO    = 2,   /* HDA, USB audio, I2S                    */
    SIGMA_DEV_GPU      = 3,   /* framebuffer, DRM/KMS, compute          */
    SIGMA_DEV_INPUT    = 4,   /* keyboard, mouse, touchscreen, gamepad  */
    SIGMA_DEV_USB      = 5,   /* USB hub + generic USB devices          */
    SIGMA_DEV_I2C      = 6,   /* sensors, touch controllers, PMICs      */
    SIGMA_DEV_SPI      = 7,   /* flash, displays, SPI peripherals       */
    SIGMA_DEV_PLATFORM = 8,   /* hardcoded board devices (ACPI/DT)      */
    SIGMA_DEV_VIRTUAL  = 9,   /* loopback, virtio, nullblk              */
} sigma_dev_class_t;

typedef enum {
    SIGMA_BUS_PCI      = 0,
    SIGMA_BUS_USB      = 1,
    SIGMA_BUS_I2C      = 2,
    SIGMA_BUS_SPI      = 3,
    SIGMA_BUS_PLATFORM = 4,
    SIGMA_BUS_VIRTUAL  = 5,
} sigma_bus_type_t;

typedef enum {
    SIGMA_POWER_ACTIVE  = 0,
    SIGMA_POWER_IDLE    = 1,   /* D1/D2 — reduced power                  */
    SIGMA_POWER_SUSPEND = 2,   /* D3hot — context preserved in RAM        */
    SIGMA_POWER_OFF     = 3,   /* D3cold — no power                       */
} sigma_power_state_t;

/* ── Core device node ────────────────────────────────────────────────────── */
typedef struct sigma_device {
    char               name[64];
    sigma_dev_class_t  class_;
    sigma_bus_type_t   bus;
    sigma_power_state_t power_state;
    struct sigma_device* parent;   /* device tree relationship              */
    void*              driver_data; /* driver-private                       */
    sigma_u64          device_id;  /* PCI: vendor<<32|device; USB: vid<<16|pid */
    sigma_u32          irq;        /* assigned IRQ line                     */
    sigma_u64          mmio_base;  /* BAR0 physical address                 */
    sigma_size_t       mmio_size;
    bool               bound;      /* true when a driver has probed it      */
} sigma_device_t;

/* ── Core driver descriptor ──────────────────────────────────────────────── */
typedef struct sigma_driver {
    const char*   name;
    sigma_u64*    id_table;   /* NULL-terminated list of device_ids        */
    /* Lifecycle callbacks */
    int  (*probe)  (sigma_device_t* dev);
    void (*remove) (sigma_device_t* dev);
    int  (*suspend)(sigma_device_t* dev, sigma_power_state_t target);
    int  (*resume) (sigma_device_t* dev);
    /* SigmaOS-unique: Dilithium3 signature of this driver binary */
    sigma_u8  dilithium_sig[4595];
    bool      trusted;        /* set by kernel after verifying sig         */
} sigma_driver_t;

/* ── Driver registration ─────────────────────────────────────────────────── */
int  sigma_driver_register(sigma_driver_t* drv);
void sigma_driver_unregister(sigma_driver_t* drv);
int  sigma_device_add(sigma_device_t* dev);   /* triggers probe() on match  */
void sigma_device_remove(sigma_device_t* dev);

/* ── Async I/O Request Packets (Windows IRP model) ──────────────────────── */
typedef enum {
    SIGMA_IRP_READ   = 0,
    SIGMA_IRP_WRITE  = 1,
    SIGMA_IRP_IOCTL  = 2,
    SIGMA_IRP_POWER  = 3,
    SIGMA_IRP_PNP    = 4,
} sigma_irp_type_t;

typedef struct sigma_irp {
    sigma_irp_type_t  type;
    sigma_u64         offset;   /* byte offset for READ/WRITE              */
    sigma_size_t      length;
    void*             buffer;   /* kernel-locked buffer                    */
    int               status;   /* 0=pending, >0=done OK, <0=error         */
    void (*complete)(struct sigma_irp* irp, int result);
    void*             context;
} sigma_irp_t;

int  sigma_irp_submit(sigma_device_t* dev, sigma_irp_t* irp);
void sigma_irp_cancel(sigma_irp_t* irp);

/* ── Driver families (typed base, like macOS IOKit) ──────────────────────── */

typedef struct {
    sigma_driver_t base;
    int  (*send)      (sigma_device_t*, const void* buf, sigma_size_t len);
    int  (*recv)      (sigma_device_t*, void* buf, sigma_size_t max);
    int  (*set_mac)   (sigma_device_t*, const sigma_u8 mac[6]);
    void (*link_state)(sigma_device_t*, bool up);
} sigma_net_driver_t;

typedef struct {
    sigma_driver_t base;
    int      (*read_sectors) (sigma_device_t*, sigma_u64 lba, sigma_u32 n, void* buf);
    int      (*write_sectors)(sigma_device_t*, sigma_u64 lba, sigma_u32 n, const void* buf);
    int      (*flush)        (sigma_device_t*);
    sigma_u64 (*capacity)    (sigma_device_t*);
    sigma_u32 sector_size;
} sigma_block_driver_t;

typedef struct {
    sigma_driver_t base;
    int  (*set_mode) (sigma_device_t*, sigma_u32 w, sigma_u32 h, sigma_u32 fmt);
    void*(*map_fb)   (sigma_device_t*);
    int  (*flip)     (sigma_device_t*);
    int  (*vsync)    (sigma_device_t*);
} sigma_gpu_driver_t;

/* ── Linux shim macro (drop in upstream Linux drivers) ──────────────────── */
/* Usage: SIGMA_LINUX_DRIVER(e1000e, e1000e_pci_tbl, e1000e_probe, e1000e_remove); */
#define SIGMA_LINUX_DRIVER(drv_name, id_tbl, probe_fn, remove_fn)          \
    static sigma_driver_t _sigma_##drv_name = {                             \
        .name     = #drv_name,                                              \
        .id_table = (sigma_u64*)(id_tbl),                                   \
        .probe    = (int(*)(sigma_device_t*))probe_fn,                      \
        .remove   = (void(*)(sigma_device_t*))remove_fn,                    \
    };                                                                       \
    static void __attribute__((constructor))                                 \
    _sigma_register_##drv_name(void) {                                       \
        sigma_driver_register(&_sigma_##drv_name);                          \
    }
