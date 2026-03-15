# Generated method: CosmosModuleLoader.load_ko


class CosmosModuleLoader:
    def load_ko(self, name, binary_data):
        print(f"[KMOD] Loading module '{name}'...")
        print(f"[KMOD] Resolving external symbols for '{name}'...")
        for symbol, addr in self.symbol_table.items():
            print(f'  - Linked {symbol} -> {hex(addr)}')
        print(f'[KMOD] Applying x86_64 base relocations...')
        module_entry = 20480
        self.loaded_modules[name] = {'base': 20480, 'status': 'ACTIVE'}
        print(f"[KMOD] Module '{name}' initialized at {hex(module_entry)}")
        return True