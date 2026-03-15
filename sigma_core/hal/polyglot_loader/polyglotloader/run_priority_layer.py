# Generated method: PolyglotLoader.run_priority_layer
import os
import subprocess
import platform
from typing import Dict

class PolyglotLoader:
    def run_priority_layer(self, layer_name: str, bin_name: str):
        """Attempts to run the native binary; falls back to simulated logic."""
        ext = '.exe' if self.os_type == 'Windows' else ''
        full_path = os.path.join(self.bin_path, f'{bin_name}{ext}')
        print(f'[POLYGLOT] Initiating Layer: {layer_name} (Priority: HIGH)')
        if os.path.exists(full_path):
            try:
                result = subprocess.run([full_path], capture_output=True, text=True, timeout=5)
                print(f'[POLYGLOT] Native Execution Successful:\n{result.stdout}')
                self.status[layer_name] = 'NATIVE_ACTIVE'
                return True
            except Exception as e:
                print(f'[POLYGLOT] Native Execution Failed: {e}')
        print(f'[POLYGLOT] Warning: Native {bin_name} not found. Using Python-Safe Fallback.')
        self.status[layer_name] = 'PYTHON_FALLBACK'
        return False