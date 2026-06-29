#include "linux_compat.h"

// Example stub for e1000 network driver
static int e1000_probe(struct pci_dev *pdev, const struct pci_device_id *ent) {
    int err;

    err = pci_enable_device(pdev);
    if (err)
        return err;

    pr_info("e1000: probe successful, device enabled\n");
    return 0;
}

static void e1000_remove(struct pci_dev *pdev) {
    pci_disable_device(pdev);
    pr_info("e1000: device removed\n");
}

static struct pci_driver e1000_driver = {
    .name     = "e1000",
    .id_table = NULL,
    .probe    = e1000_probe,
    .remove   = e1000_remove,
};

int e1000_init_module(void) {
    pr_info("e1000: Intel(R) PRO/1000 Network Driver\n");
    return pci_register_driver(&e1000_driver);
}

void e1000_cleanup_module(void) {
    pci_unregister_driver(&e1000_driver);
    pr_info("e1000: driver unloaded\n");
}
