#ifndef SIGMA_LINUX_COMPAT_H
#define SIGMA_LINUX_COMPAT_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Mock Linux API Types
typedef unsigned int gfp_t;
#define GFP_KERNEL 0
#define GFP_ATOMIC 1

// Allocation functions
void *kmalloc(size_t size, gfp_t flags);
void kfree(const void *objp);

// Interrupts
typedef irqreturn_t (*irq_handler_t)(int, void *);
int request_irq(unsigned int irq, irq_handler_t handler, unsigned long flags, const char *name, void *dev);
void free_irq(unsigned int irq, void *dev_id);

// PCI Subsystem
struct pci_dev {
    unsigned int devfn;
    unsigned short vendor;
    unsigned short device;
    unsigned short subsystem_vendor;
    unsigned short subsystem_device;
    unsigned int class;
};

struct pci_driver {
    const char *name;
    const struct pci_device_id *id_table;
    int  (*probe)  (struct pci_dev *dev, const struct pci_device_id *id);
    void (*remove) (struct pci_dev *dev);
};

int pci_register_driver(struct pci_driver *drv);
void pci_unregister_driver(struct pci_driver *drv);
int pci_enable_device(struct pci_dev *dev);
void pci_disable_device(struct pci_dev *dev);

// Logging
#define printk(...) do { /* route to sigma logger */ } while(0)
#define pr_info(...) printk(__VA_ARGS__)
#define pr_err(...) printk(__VA_ARGS__)

#ifdef __cplusplus
}
#endif

#endif // SIGMA_LINUX_COMPAT_H
