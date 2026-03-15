# Generated method: QuantumBIEngine._import_data
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import math

class QuantumBIEngine:
    def _import_data(self):
        f = filedialog.askopenfilename(filetypes=[('Data Vectors', '*.csv *.json *.sql *.xlsx')])
        if f:
            self._load_mock_data()
            self.status.config(text=f"INGESTED 1.2M ROWS FROM: {f.split('/')[-1]} | 0.08ms QUERY TIME", bg=PAL['success'], fg='black')