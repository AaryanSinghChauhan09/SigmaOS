# Generated method: NCERTGrapher.plot
import tkinter as tk
from tkinter import ttk
import math

class NCERTGrapher:
    def plot(self):
        if not hasattr(self, 'canvas') or not self.canvas.winfo_exists():
            return
        self.canvas.delete('all')
        w = self.canvas.winfo_width()
        h = self.canvas.winfo_height()
        if w < 100:
            w, h = (960, 500)
        cx, cy = (w / 2, h / 2)
        self.canvas.create_line(0, cy, w, cy, fill='#252840')
        self.canvas.create_line(cx, 0, cx, h, fill='#252840')
        rng = self.range_var.get()
        scale_x = cx / rng
        scale_y = cy / (rng / 2)
        expr = self.func_var.get().replace('^', '**')
        points = []
        safe_dict = {'x': 0, 'sin': math.sin, 'cos': math.cos, 'tan': math.tan, 'exp': math.exp, 'log': math.log, 'sqrt': math.sqrt, 'pi': math.pi, 'e': math.e}
        step = rng / 200
        for i in range(-200, 201):
            x = i * step
            safe_dict['x'] = x
            try:
                y = eval(expr, {'__builtins__': None}, safe_dict)
                px = cx + x * scale_x
                py = cy - y * scale_y
                if 0 <= px <= w and 0 <= py <= h:
                    points.append((px, py))
            except:
                continue
        if len(points) > 1:
            self.canvas.create_line(points, fill='#00D26A', width=2, smooth=True)
        self.canvas.create_text(w - 20, cy + 15, text='x', fill='white')
        self.canvas.create_text(cx + 15, 20, text='y', fill='white')