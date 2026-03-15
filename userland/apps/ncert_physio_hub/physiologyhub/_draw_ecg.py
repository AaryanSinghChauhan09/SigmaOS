# Generated method: PhysiologyHub._draw_ecg
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _draw_ecg(self):
        self.ecg_canvas.delete('all')
        points = [(0, 100)]
        x = 0
        for _ in range(10):
            for i in range(20):
                points.append((x + i, 100 - 10 * math.sin(i * 0.15)))
            x += 20
            points.append((x, 100))
            points.append((x + 5, 110))
            points.append((x + 10, 40))
            points.append((x + 15, 110))
            points.append((x + 20, 100))
            x += 20
            for i in range(30):
                points.append((x + i, 100 - 15 * math.sin(i * 0.1)))
            x += 50
        self.ecg_canvas.create_line(points, fill='#00FF00', width=2)