"""
SigmaOS Sovereign Polyglot Loader (v1.0 Apex)
==============================================
USP: Orchestrates multi-language core components.
Prioritizes Low-Level (C/Rust) over Middle-Level (Go/Python).
"""

import os
import subprocess
import platform
from typing import Dict

class PolyglotLoader:
    def __init__(self):
        self.os_type = platform.system()
        self.bin_path = os.path.join(os.path.dirname(__file__), "native_bin")
        self.status = {}
        self.active_cores: Dict[str, str] = {}

    def hot_swap_core(self, layer: str, profile: str):
        """USP: Dynamically swaps native binary cores based on OS profile."""
        # Mapping: layer -> {profile: binary}
        registry = {
            "memory": {"APEX_GAMING": "mem_fast_rs", "SUSTAINABLE": "mem_green_rs"},
            "ipc":    {"NEURAL_RESEARCH": "ipc_neural_go", "STANDARD": "ipc_std_go"}
        }
        
        target_bin = registry.get(layer, {}).get(profile)
        if target_bin:
            self.active_cores[layer] = target_bin
            print(f"[POLYGLOT] Hot-Swap: Layer '{layer}' re-core to '{target_bin}'")
            return self.run_priority_layer(layer, target_bin)
        return False

    def run_priority_layer(self, layer_name: str, bin_name: str):
        """Attempts to run the native binary; falls back to simulated logic."""
        ext = ".exe" if self.os_type == "Windows" else ""
        full_path = os.path.join(self.bin_path, f"{bin_name}{ext}")

        print(f"[POLYGLOT] Initiating Layer: {layer_name} (Priority: HIGH)")
        
        if os.path.exists(full_path):
            try:
                result = subprocess.run([full_path], capture_output=True, text=True, timeout=5)
                print(f"[POLYGLOT] Native Execution Successful:\n{result.stdout}")
                self.status[layer_name] = "NATIVE_ACTIVE"
                return True
            except Exception as e:
                print(f"[POLYGLOT] Native Execution Failed: {e}")
        
        print(f"[POLYGLOT] Warning: Native {bin_name} not found. Using Python-Safe Fallback.")
        self.status[layer_name] = "PYTHON_FALLBACK"
        return False

    def get_health_report(self):
        return self.status

# Singleton Instance for Kernel Access
SigmaPolyglot = PolyglotLoader()
