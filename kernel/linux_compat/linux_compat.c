#include "linux_compat.h"

// Basic memory allocator mapping to SigmaOS kernel allocator
void *kmalloc(size_t size, gfp_t flags) {
    // SigmaOS memory allocation syscall/internal function
    // For now, returning a mock pointer
    return (void*)0xDEADBEEF; 
}

void kfree(const void *objp) {
    // SigmaOS free
}

// Basic interrupt request mapping
int request_irq(unsigned int irq, irq_handler_t handler, unsigned long flags, const char *name, void *dev) {
    // Route to sigma-bus interrupt controller
    return 0;
}

void free_irq(unsigned int irq, void *dev_id) {
    // Route to sigma-bus interrupt controller
}

// PCI device registration mapping
int pci_register_driver(struct pci_driver *drv) {
    // Register driver with SigmaOS device manager
    return 0;
}

void pci_unregister_driver(struct pci_driver *drv) {
    // Unregister
}

int pci_enable_device(struct pci_dev *dev) {
    return 0;
}

void pci_disable_device(struct pci_dev *dev) {
}
