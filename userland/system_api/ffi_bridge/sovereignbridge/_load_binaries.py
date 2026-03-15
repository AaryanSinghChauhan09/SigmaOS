# Generated method: SovereignBridge._load_binaries
import os
import sys
import ctypes
from pathlib import Path

class SovereignBridge:
    def _load_binaries(self):
        """Attempts to load compiled SO/DLL binaries for C and Rust cores."""
        root = Path(__file__).resolve().parent
        lib_ext = '.dll' if os.name == 'nt' else '.so'
        c_path = root.parent.parent / 'kernel' / f'sovereign_core{lib_ext}'
        rust_path = root.parent.parent / 'kernel' / f'libvanguard{lib_ext}'
        self.emulated = not (c_path.exists() and rust_path.exists())
        if not self.emulated:
            try:
                self._c_lib = ctypes.CDLL(str(c_path))
                self._rust_lib = ctypes.CDLL(str(rust_path))
                self._log_event('BRIDGE', 'Sovereign Low-Level Cores LINKED successfully.')
            except Exception as e:
                self._log_error('BRIDGE', f'Binary link failed: {e}. Falling back to Neural Emulation.')
                self.emulated = True
        else:
            self._log_event('BRIDGE', 'No native binaries found. Operating in Sovereign Emulation Mode.')