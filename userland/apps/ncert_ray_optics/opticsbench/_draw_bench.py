# Generated method: OpticsBench._draw_bench
import tkinter as tk
from tkinter import ttk

class OpticsBench:
    def _draw_bench(self, u, v, f, mode):
        self.canvas.delete('all')
        cx, cy = (500, 250)
        self.canvas.create_line(50, cy, 950, cy, fill='#333', dash=(4, 4))
        self.canvas.create_line(cx, cy - 100, cx, cy + 100, fill='white', width=2)
        self.canvas.create_text(cx, cy + 120, text=mode, fill='white')
        ox = cx + u * 4
        self.canvas.create_line(ox, cy, ox, cy - 50, fill=PAL['object'], width=4, arrow=tk.LAST)
        self.canvas.create_text(ox, cy + 20, text='OBJECT', fill=PAL['object'])
        if abs(v) < 1000:
            scale_factor = 4 if 'Lens' in mode else -4
            ix = cx + v * scale_factor
            val_h = 50 * (v / u if 'Lens' in mode else -v / u)
            self.canvas.create_line(ix, cy, ix, cy + val_h, fill=PAL['image'], width=4, arrow=tk.LAST)
            self.canvas.create_text(ix, cy + 70 if val_h > 0 else cy - 70, text='IMAGE', fill=PAL['image'])