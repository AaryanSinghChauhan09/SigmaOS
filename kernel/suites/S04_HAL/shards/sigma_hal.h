/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S04_HAL/shards/sigma_hal.h
 * =========================================================================
 * Hardware Abstraction Layer — gap-closes:
 *   Linux  : platform_device, of_device, ACPI, irqdomain
 *   macOS  : IOKit, DriverKit, HIDKitDriverKit
 *   Windows: WDM, HAL.dll, Plug-and-Play Manager
 *   BSD    : bus_space, bus_dma, newbus
 *   RTOS   : FreeRTOS port layer, Zephyr devicetree
 * =========================================================================
 */

#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

typedef unsigned long long hal_u64;
typedef unsigned int       hal_u32;
typedef unsigned short     hal_u16;
typedef unsigned char      hal_u8;
typedef signed   int       hal_i32;
typedef unsigned char      hal_bool;
#define HAL_TRUE  ((hal_bool)1)
#define HAL_FALSE ((hal_bool)0)
#define HAL_NULL  ((void*)0)
#define HAL_OK    ((hal_i32) 0)
#define HAL_ERR   ((hal_i32)-1)

/* ── Device bus types ────────────────────────────────────────────────────── */
typedef enum {
    BUS_PLATFORM = 0,  /* SoC-integrated, no enumeration needed         */
    BUS_PCI      = 1,  /* PCIe / PCI                                    */
    BUS_USB      = 2,  /* USB 2/3/4                                     */
    BUS_I2C      = 3,  /* I²C / SMBus                                   */
    BUS_SPI      = 4,  /* SPI                                           */
    BUS_VIRTIO   = 5,  /* VirtIO (QEMU/cloud hypervisor)                */
    BUS_ACPI     = 6   /* ACPI-enumerated                               */
} sigma_bus_t;

/* ── Device class ────────────────────────────────────────────────────────── */
typedef enum {
    DEV_BLOCK   = 0,   /* disk, NVMe, eMMC                              */
    DEV_NET     = 1,   /* NIC, WiFi                                     */
    DEV_INPUT   = 2,   /* keyboard, mouse, touchscreen                  */
    DEV_DISPLAY = 3,   /* framebuffer, DRM/KMS, DisplayPort             */
    DEV_AUDIO   = 4,   /* sound card, codec                             */
    DEV_SERIAL  = 5,   /* UART, tty                                     */
    DEV_MISC    = 6    /* watchdog, RTC, TPM, PMU                       */
} sigma_dev_class_t;

/* ── IRQ trigger types ───────────────────────────────────────────────────── */
typedef enum {
    IRQ_EDGE_RISING  = 0,
    IRQ_EDGE_FALLING = 1,
    IRQ_LEVEL_HIGH   = 2,
    IRQ_LEVEL_LOW    = 3,
    IRQ_MSI          = 4   /* Message Signaled Interrupt (PCIe)          */
} sigma_irq_type_t;

#define SIGMA_HAL_MAX_DEVICES 256
#define SIGMA_HAL_MAX_IRQS    512
#define SIGMA_DEV_NAME_LEN     48

/* ── Interrupt handler ───────────────────────────────────────────────────── */
typedef void (*sigma_irq_handler_t)(hal_u32 irq, void *dev_id);

/* ── IRQ descriptor ─────────────────────────────────────────────────────── */
typedef struct {
    hal_u32            irq_num;
    sigma_irq_type_t   type;
    sigma_irq_handler_t handler;
    void              *dev_id;
    hal_u64            count;       /* interrupt count telemetry          */
    hal_bool           enabled;
} sigma_irq_t;

/* ── Device operations (vnops equivalent for devices) ───────────────────── */
typedef struct sigma_device_s sigma_device_t;
typedef struct {
    hal_i32  (*probe)(sigma_device_t *dev);
    hal_i32  (*init)(sigma_device_t *dev);
    void     (*remove)(sigma_device_t *dev);
    hal_i32  (*suspend)(sigma_device_t *dev);
    hal_i32  (*resume)(sigma_device_t *dev);
    hal_i32  (*ioctl)(sigma_device_t *dev, hal_u32 cmd, hal_u64 arg);
} sigma_dev_ops_t;

/* ── Device descriptor ──────────────────────────────────────────────────── */
struct sigma_device_s {
    char               name[SIGMA_DEV_NAME_LEN];
    hal_u32            id;
    sigma_bus_t        bus;
    sigma_dev_class_t  cls;
    hal_u32            vendor_id;
    hal_u32            device_id;
    hal_u64            base_addr;    /* MMIO/IO-port base                */
    hal_u32            irq;
    sigma_dev_ops_t   *ops;
    void              *driver_data;  /* driver-private state             */
    hal_bool           online;
};

/* ── DMA buffer (bus_dma equivalent) ────────────────────────────────────── */
typedef struct {
    hal_u64  phys_addr;
    void    *virt_addr;
    hal_u64  size;
    hal_bool coherent;  /* cache-coherent mapping                       */
} sigma_dma_buf_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void  sigma_hal_init(void);

/* Device registry */
hal_i32  sigma_hal_register(sigma_device_t *dev);
void     sigma_hal_unregister(hal_u32 dev_id);
sigma_device_t *sigma_hal_find(const char *name);
void     sigma_hal_enumerate_bus(sigma_bus_t bus);
void     sigma_hal_device_list(void);

/* IRQ management (Linux irqdomain / Windows IoConnectInterrupt) */
hal_i32  sigma_irq_request(hal_u32 irq, sigma_irq_type_t type,
                            sigma_irq_handler_t handler, void *dev_id);
void     sigma_irq_free(hal_u32 irq);
void     sigma_irq_enable(hal_u32 irq);
void     sigma_irq_disable(hal_u32 irq);
void     sigma_irq_dispatch(hal_u32 irq);   /* called from arch irq vector */
void     sigma_irq_stats(void);

/* MMIO helpers */
hal_u32  sigma_mmio_read32(hal_u64 addr);
void     sigma_mmio_write32(hal_u64 addr, hal_u32 val);

/* DMA (bus_dma_alloc equivalent) */
hal_i32  sigma_dma_alloc(sigma_dma_buf_t *buf, hal_u64 size, hal_bool coherent);
void     sigma_dma_free(sigma_dma_buf_t *buf);

/* Power management (Linux PM runtime / Windows D-states) */
hal_i32  sigma_pm_suspend_device(hal_u32 dev_id);
hal_i32  sigma_pm_resume_device(hal_u32 dev_id);

#endif /* SIGMA_HAL_H */
