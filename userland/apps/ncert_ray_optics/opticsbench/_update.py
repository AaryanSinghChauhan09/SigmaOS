# Generated method: OpticsBench._update
import tkinter as tk
from tkinter import ttk

class OpticsBench:
    def _update(self):
        mode = self.mode.get()
        f = float(self.f_scale.get())
        u = -float(self.u_scale.get())
        if 'Mirror' in mode:
            actual_f = -f if 'Concave' in mode else f
            try:
                v = 1 / (1 / actual_f - 1 / u)
                m = -v / u
            except ZeroDivisionError:
                v = float('inf')
                m = 0
        else:
            actual_f = f if 'Convex' in mode else -f
            try:
                v = 1 / (1 / actual_f + 1 / u)
                m = v / u
            except ZeroDivisionError:
                v = float('inf')
                m = 0
        nature = 'REAL' if v > 0 or ('Mirror' in mode and v < 0) else 'VIRTUAL'
        if 'Mirror' in mode:
            nature = 'REAL' if v < 0 else 'VIRTUAL'
            pos = 'In front' if v < 0 else 'Behind'
        else:
            nature = 'REAL' if v > 0 else 'VIRTUAL'
            pos = 'Other side' if v > 0 else 'Same side'
        self.out_lbl.config(text=f'Image dist (v): {abs(v):.2f} units | Magnification: {m:.2f} | Nature: {nature} ({pos})')
        self._draw_bench(u, v, f, mode)