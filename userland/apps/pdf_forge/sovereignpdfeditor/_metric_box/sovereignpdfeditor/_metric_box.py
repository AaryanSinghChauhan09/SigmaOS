# Generated method: SovereignPDFEditor._metric_box
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class SovereignPDFEditor:
    def _metric_box(self, parent, key, val, color):
        f = tk.Frame(parent, bg=PAL['panel'], pady=10)
        f.pack(fill='x')
        tk.Label(f, text=key, font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        tk.Label(f, text=val, font=('Inter', 10, 'bold'), fg=color, bg=PAL['panel']).pack(anchor='w')