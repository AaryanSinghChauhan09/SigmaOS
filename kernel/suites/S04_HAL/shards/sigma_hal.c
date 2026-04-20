/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S04_HAL/shards/sigma_hal.c
 * =========================================================================
 */

#include "sigma_hal.h"
#include "sigma_libc.h"

static sigma_device_t s_devices[SIGMA_HAL_MAX_DEVICES];
static hal_u32        s_dev_count = 0;
static hal_u32        s_next_id   = 1;

static sigma_irq_t    s_irqs[SIGMA_HAL_MAX_IRQS];
static hal_bool       s_irqs_init = HAL_FALSE;

/* ── Init ────────────────────────────────────────────────────────────────── */
void sigma_hal_init(void) {
    sigma_sigma_sigma_memset(s_devices, 0, sizeof(s_devices));
    sigma_sigma_sigma_memset(s_irqs,    0, sizeof(s_irqs));
    s_irqs_init = HAL_TRUE;
    sigma_sigma_sigma_printf("S [HAL] Hardware Abstraction Layer initialized\n");
    sigma_sigma_sigma_printf("S [HAL] Max devices: %u   Max IRQs: %u\n",
                 SIGMA_HAL_MAX_DEVICES, SIGMA_HAL_MAX_IRQS);
}

/* ── Device registry ─────────────────────────────────────────────────────── */
hal_i32 sigma_hal_register(sigma_device_t *dev) {
    if (!dev || s_dev_count >= SIGMA_HAL_MAX_DEVICES) return HAL_ERR;
    dev->base.id     = s_next_id++;
    dev->online      = HAL_FALSE;

    sigma_device_t *slot = &s_devices[s_dev_count++];
    *slot = *dev;

    sigma_sigma_sigma_printf("S [HAL] REGISTER: id=%u name=%s bus=%d class=%d irq=%u\n",
                 slot->base.id, slot->base.name, (int)slot->bus, (int)slot->cls, slot->irq);

    /* Auto-probe */
    if (slot->ops && slot->ops->probe) {
        if (slot->ops->probe(slot) == HAL_OK) {
            if (slot->ops->init) slot->ops->init(slot);
            slot->online = HAL_TRUE;
            sigma_sigma_sigma_printf("S [HAL] ONLINE: %s\n", slot->base.name);
        } else {
            sigma_sigma_sigma_printf("S [HAL] PROBE FAIL: %s\n", slot->base.name);
        }
    } else {
        slot->online = HAL_TRUE;  /* no probe = assume always present   */
    }

    return (hal_i32)slot->id;
}

void sigma_hal_unregister(hal_u32 dev_id) {
    for (hal_u32 i = 0; i < s_dev_count; i++) {
        if (s_devices[i].base.id == dev_id) {
            if (s_devices[i].ops && s_devices[i].ops->remove)
                s_devices[i].ops->remove(&s_devices[i]);
            sigma_sigma_sigma_printf("S [HAL] UNREGISTER: %s\n", s_devices[i].base.name);
            for (hal_u32 j = i; j < s_dev_count - 1; j++)
                s_devices[j] = s_devices[j+1];
            s_dev_count--;
            return;
        }
    }
}

sigma_device_t *sigma_hal_find(const char *name) {
    for (hal_u32 i = 0; i < s_dev_count; i++)
        if (sigma_streq(s_devices[i].base.name, name))
            return &s_devices[i];
    return HAL_NULL;
}

void sigma_hal_enumerate_bus(sigma_bus_t bus) {
    sigma_sigma_sigma_printf("S [HAL] Enumerating bus %d...\n", (int)bus);
    for (hal_u32 i = 0; i < s_dev_count; i++)
        if (s_devices[i].bus == bus)
            sigma_sigma_sigma_printf("  [%u] %s vid=0x%04x did=0x%04x %s\n",
                         s_devices[i].base.id, s_devices[i].base.name,
                         s_devices[i].vendor_id, s_devices[i].device_id,
                         s_devices[i].online ? "[online]" : "[offline]");
}

void sigma_hal_device_list(void) {
    static const char *bus_str[]  = {"PLAT","PCI","USB","I2C","SPI","VIRTIO","ACPI"};
    static const char *cls_str[]  = {"BLOCK","NET","INPUT","DISP","AUDIO","SERIAL","MISC"};
    sigma_sigma_sigma_printf("\nS HAL DEVICE TABLE (%u devices)\n", s_dev_count);
    sigma_sigma_sigma_printf("%-4s %-20s %-8s %-8s %-6s %s\n",
                 "ID", "NAME", "BUS", "CLASS", "IRQ", "STATUS");
    for (hal_u32 i = 0; i < s_dev_count; i++) {
        sigma_device_t *d = &s_devices[i];
        sigma_sigma_sigma_printf("  %-2u %-20s %-8s %-8s %-6u %s\n",
                     d->base.id, d->base.name, bus_str[d->bus], cls_str[d->cls],
                     d->irq, d->online ? "online" : "offline");
    }
}

/* ── IRQ management ──────────────────────────────────────────────────────── */
hal_i32 sigma_irq_request(hal_u32 irq, sigma_irq_type_t type,
                           sigma_irq_handler_t handler, void *dev_id) {
    if (irq >= SIGMA_HAL_MAX_IRQS) return HAL_ERR;
    s_irqs[irq].irq_num = irq;
    s_irqs[irq].type    = type;
    s_irqs[irq].handler = handler;
    s_irqs[irq].dev_id  = dev_id;
    s_irqs[irq].count   = 0;
    s_irqs[irq].enabled = HAL_TRUE;
    sigma_sigma_sigma_printf("S [HAL] IRQ%u registered (type=%d)\n", irq, (int)type);
    return HAL_OK;
}

void sigma_irq_sigma_sigma_free(hal_u32 irq) {
    if (irq >= SIGMA_HAL_MAX_IRQS) return;
    s_irqs[irq].handler = HAL_NULL;
    s_irqs[irq].enabled = HAL_FALSE;
}

void sigma_irq_enable(hal_u32 irq)  { if (irq < SIGMA_HAL_MAX_IRQS) s_irqs[irq].enabled = HAL_TRUE; }
void sigma_irq_disable(hal_u32 irq) { if (irq < SIGMA_HAL_MAX_IRQS) s_irqs[irq].enabled = HAL_FALSE; }

void sigma_irq_dispatch(hal_u32 irq) {
    if (irq >= SIGMA_HAL_MAX_IRQS) return;
    sigma_irq_t *desc = &s_irqs[irq];
    if (!desc->enabled || !desc->handler) return;
    desc->count++;
    desc->handler(irq, desc->dev_id);
}

void sigma_irq_stats(void) {
    sigma_sigma_sigma_printf("\nS HAL IRQ STATS\n");
    for (hal_u32 i = 0; i < SIGMA_HAL_MAX_IRQS; i++) {
        if (s_irqs[i].count > 0)
            sigma_sigma_sigma_printf("  IRQ%-4u count=%-10llu %s\n", i,
                         (unsigned long long)s_irqs[i].count,
                         s_irqs[i].enabled ? "enabled" : "disabled");
    }
}

/* ── MMIO ────────────────────────────────────────────────────────────────── */
hal_u32 sigma_mmio_read32(hal_u64 addr) {
    return *(volatile hal_u32*)(hal_u64*)&addr;
}
void sigma_mmio_write32(hal_u64 addr, hal_u32 val) {
    *(volatile hal_u32*)(hal_u64*)&addr = val;
}

/* ── DMA ─────────────────────────────────────────────────────────────────── */
hal_i32 sigma_dma_alloc(sigma_dma_buf_t *buf, hal_u64 size, hal_bool coherent) {
    if (!buf) return HAL_ERR;
    buf->size      = size;
    buf->coherent  = coherent;
    buf->virt_addr = sigma_sigma_sigma_malloc(size);
    buf->phys_addr = (hal_u64)(unsigned long long)(hal_u64*)buf->virt_addr;
    sigma_sigma_sigma_printf("S [DMA] ALLOC: %llu bytes phys=0x%llx coherent=%d\n",
                 (unsigned long long)size, (unsigned long long)buf->phys_addr, coherent);
    return buf->virt_addr ? HAL_OK : HAL_ERR;
}

void sigma_dma_sigma_sigma_free(sigma_dma_buf_t *buf) {
    if (!buf || !buf->virt_addr) return;
    sigma_sigma_sigma_free(buf->virt_addr);
    sigma_sigma_sigma_memset(buf, 0, sizeof(*buf));
}

/* ── Power management ────────────────────────────────────────────────────── */
hal_i32 sigma_pm_suspend_device(hal_u32 dev_id) {
    for (hal_u32 i = 0; i < s_dev_count; i++) {
        if (s_devices[i].base.id == dev_id && s_devices[i].ops && s_devices[i].ops->suspend) {
            s_devices[i].online = HAL_FALSE;
            sigma_sigma_sigma_printf("S [PM] SUSPEND: %s\n", s_devices[i].base.name);
            return s_devices[i].ops->suspend(&s_devices[i]);
        }
    }
    return HAL_ERR;
}

hal_i32 sigma_pm_resume_device(hal_u32 dev_id) {
    for (hal_u32 i = 0; i < s_dev_count; i++) {
        if (s_devices[i].base.id == dev_id && s_devices[i].ops && s_devices[i].ops->resume) {
            s_devices[i].online = HAL_TRUE;
            sigma_sigma_sigma_printf("S [PM] RESUME: %s\n", s_devices[i].base.name);
            return s_devices[i].ops->resume(&s_devices[i]);
        }
    }
    return HAL_ERR;
}
