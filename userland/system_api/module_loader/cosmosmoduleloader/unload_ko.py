# Generated method: CosmosModuleLoader.unload_ko


class CosmosModuleLoader:
    def unload_ko(self, name):
        if name in self.loaded_modules:
            print(f"[KMOD] Unloading module '{name}'...")
            del self.loaded_modules[name]
            return True
        return False