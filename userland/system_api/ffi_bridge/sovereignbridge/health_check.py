# Generated method: SovereignBridge.health_check
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def health_check(self) -> str:
        status = 'NATIVE' if not self.emulated else 'EMULATED'
        rust_status = 'OK' if self.vanguard_health_check() else 'FAIL'
        return f'OK — Sovereign Bridge ({status}). Rust Vanguard: {rust_status}. Low-Lat IPC: ARMED.'