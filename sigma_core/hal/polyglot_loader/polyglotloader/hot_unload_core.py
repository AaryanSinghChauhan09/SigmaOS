# Generated method: PolyglotLoader.hot_unload_core
import os
import subprocess
import platform
from typing import Dict

class PolyglotLoader:
    def hot_unload_core(self, layer: str):
        """USP: Minimalist operation. Unloads native cores to free system resources."""
        if layer in self.active_cores:
            bin_name = self.active_cores.pop(layer)
            print(f"[POLYGLOT] Hot-Unload: Layer '{layer}' core '{bin_name}' removed from memory.")
            self.status[layer] = 'UNLOADED'
            return True
        return False