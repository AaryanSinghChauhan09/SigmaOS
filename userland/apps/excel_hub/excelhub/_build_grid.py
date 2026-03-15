"""
Auto-split from userland\apps\excel_hub.py — ExcelHub._build_grid
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time



class ExcelHub:
    def _build_grid(self, parent):
        cols = ('A', 'B', 'C', 'D', 'E')
        self.tree = ttk.Treeview(parent, columns=cols, show='headings')
        for char in cols:
            self.tree.heading(char, text=f'COLUMN_{char}')
            self.tree.column(char, width=150, anchor='center')
        for i in range(50):
            vals = [f'Data_{i}_{j}' for j in range(5)]
            if i % 5 == 0:
                vals[4] = f'PREDICTED_{random.randint(100, 999)}'
            self.tree.insert('', 'end', values=vals)
        self.tree.pack(fill='both', expand=True)
