# Generated method: ProjectFlow._init_gantt
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _init_gantt(self, parent):
        canvas = tk.Canvas(parent, bg='#000', highlightthickness=0)
        canvas.pack(fill='both', expand=True, padx=20, pady=20)
        for i in range(10):
            x = 100 + i * 100
            canvas.create_line(x, 0, x, 800, fill='#1C1C24')
            canvas.create_text(x, 20, text=f'Wk {i + 1}', fill=PAL['dim'], font=('Inter', 8))
        tasks = [('Core Kernel', 50, 200, PAL['accent']), ('UI Refactor', 250, 450, PAL['secondary']), ('Security Audit', 400, 700, PAL['success'])]
        for i, (name, x1, x2, color) in enumerate(tasks):
            y = 100 + i * 60
            canvas.create_text(50, y + 10, text=name, fill='white', anchor='e', font=('Inter', 9))
            canvas.create_rectangle(x1, y, x2, y + 20, fill=color, outline='')