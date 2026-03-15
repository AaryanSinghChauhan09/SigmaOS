# Generated method: NumberBaseConverter._on_change
import tkinter as tk
from tkinter import ttk

class NumberBaseConverter:
    def _on_change(self, source_key):
        try:
            v_str, base = self._vars[source_key]
            val_str = v_str[0].get().strip()
            if not val_str:
                return
            n = int(val_str, base)
            for key, (v, b) in self._vars.items():
                if key == source_key:
                    continue
                v[0].trace_remove('write', v[0].trace_info()[0][1]) if v[0].trace_info() else None
            for key, (v, b) in self._vars.items():
                if key == source_key:
                    continue
                rep = bin(n)[2:] if b == 2 else oct(n)[2:] if b == 8 else hex(n)[2:].upper() if b == 16 else str(n)
                try:
                    self._entries[key].delete(0, 'end')
                    self._entries[key].insert(0, rep)
                except Exception:
                    pass
        except (ValueError, tk.TclError):
            pass