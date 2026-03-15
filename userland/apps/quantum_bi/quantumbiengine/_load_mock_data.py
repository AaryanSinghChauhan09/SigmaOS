"""
Auto-split from userland\apps\quantum_bi.py — QuantumBIEngine._load_mock_data
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math



class QuantumBIEngine:
    def _load_mock_data(self):
        self.loaded_dataset = True
        self.dims_tree.delete(*self.dims_tree.get_children())
        for d in ['Region (Geo)', 'Product (Str)', 'Date (Time)', 'Segment (Str)']:
            self.dims_tree.insert('', 'end', values=(d,))
        self.meas_tree.delete(*self.meas_tree.get_children())
        for m in ['Revenue (Num)', 'Profit Margin (Num)', 'Units Sold (Num)', 'Churn (Pcnt)']:
            self.meas_tree.insert('', 'end', values=(m,))
        self.grid.delete(*self.grid.get_children())
        regions = ['NA', 'EMEA', 'APAC', 'LATAM']
        cats = ['Hardware', 'Sovereign-SaaS', 'Neural Cores', 'Quantum API']
        for i in range(1, 21):
            self.grid.insert('', 'end', values=(f'Tx-{8000 + i}', random.choice(regions), random.choice(cats), f'{random.randint(100, 999)},{random.randint(100, 999)}.00', f'+{random.uniform(0.5, 15.0):.1f}%'))
        self._draw_chart()
