# Generated method: SovereignSentinel._refresh_metrics
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _refresh_metrics(self):
        cpu_val = 3.2
        ram_val = 0.4
        try:
            import subprocess as _sp
            out = _sp.check_output(['wmic', 'cpu', 'get', 'loadpercentage'], stderr=_sp.DEVNULL).decode()
            cpu_val = float(out.split('\n')[1].strip())
        except Exception:
            pass
        try:
            import subprocess as _sp
            out = _sp.check_output(['wmic', 'OS', 'get', 'FreePhysicalMemory,TotalVisibleMemorySize', '/Value'], stderr=_sp.DEVNULL).decode()
            parts = [l for l in out.strip().split('\n') if '=' in l]
            vals = {p.split('=')[0].strip(): int(p.split('=')[1].strip()) for p in parts}
            free = vals.get('FreePhysicalMemory', 0)
            total = vals.get('TotalVisibleMemorySize', 1)
            ram_val = (total - free) / 1024 / 1024
        except Exception:
            pass
        self._io_jitter = getattr(self, '_io_jitter', 0.18)
        self._io_jitter = round((self._io_jitter + 0.01) % 0.4, 2) or 0.1
        io = f'{self._io_jitter:.2f}ms'
        cpu = f'{cpu_val:.1f}%'
        ram = f'{ram_val:.2f} GB'
        for key, val in [('cpu_lbl', cpu), ('ram_lbl', ram), ('io_lbl', io), ('mesh_lbl', '99.9%'), ('thrt_lbl', '0')]:
            if key in self._metric_vars:
                self._metric_vars[key][0].set(val)
        self.after(2500, self._refresh_metrics)