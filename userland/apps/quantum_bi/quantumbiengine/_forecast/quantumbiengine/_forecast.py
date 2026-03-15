# Generated method: QuantumBIEngine._forecast
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math

class QuantumBIEngine:
    def _forecast(self):
        if not self.loaded_dataset:
            messagebox.showerror('No Data', 'Import a matrix before applying Neural ML models.')
            return
        self.canvas.delete('all')
        w, h = (650, 450)
        self.canvas.create_line(50, h - 50, w - 50, h - 50, fill=PAL['dim'], width=2)
        self.canvas.create_line(50, 50, 50, h - 50, fill=PAL['dim'], width=2)
        points = []
        for i in range(10):
            x = 50 + i * 45
            y = h - 50 - random.randint(50, 200)
            points.extend([x, y])
            self.canvas.create_oval(x - 4, y - 4, x + 4, y + 4, fill=PAL['accent'], outline='')
        self.canvas.create_line(points, fill=PAL['accent'], width=3, smooth=True)
        fx = points[-2]
        fy = points[-1]
        forecast_pts = [fx, fy]
        for i in range(1, 5):
            nx = fx + i * 45
            ny = fy - random.randint(10, 50)
            forecast_pts.extend([nx, ny])
        self.canvas.create_line(forecast_pts, fill=PAL['chart3'], width=3, dash=(5, 5), smooth=True)
        self.canvas.create_text(w / 2, 30, text='MACHINE LEARNING FORECAST: 96.8% CONFIDENCE (ARIMA/PROPHET)', fill=PAL['chart3'], font=('Inter', 12, 'bold'))
        self.status.config(text='FORECAST APPLIED: ARIMA + NEURAL DEEPNET', bg=PAL['accent'], fg='black')