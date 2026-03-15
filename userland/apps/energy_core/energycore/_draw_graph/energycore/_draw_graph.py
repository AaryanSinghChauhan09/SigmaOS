# Generated method: EnergyCore._draw_graph
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
import time
import random
from userland.system_api.sigma_std import SigmaSys

class EnergyCore:
    def _draw_graph(self):
        self.canvas.delete('all')
        width = 500
        points = []
        for i in range(20):
            x = i * (width / 19)
            y = 100 - i * 4 + random.randint(-5, 5)
            points.extend([x, max(10, y)])
        if len(points) >= 4:
            self.canvas.create_line(points, fill=PAL['accent'], width=3, smooth=True)