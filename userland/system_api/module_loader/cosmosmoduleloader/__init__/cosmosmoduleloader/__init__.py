# Generated method: CosmosModuleLoader.__init__


class CosmosModuleLoader:
    def __init__(self, kernel):
        self.kernel = kernel
        self.loaded_modules = {}
        self.symbol_table = {'kmalloc': 4096, 'kfree': 4224, 'lisp_eval': 8192, 'pci_read': 12288, 'vfs_open': 16384}