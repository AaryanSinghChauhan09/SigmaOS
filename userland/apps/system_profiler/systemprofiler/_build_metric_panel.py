# Generated method: SystemProfiler._build_metric_panel
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def _build_metric_panel(self, parent, title, label_text, val_text, desc):
        f = tk.Frame(parent, bg=PAL['panel'], padx=20, pady=20)
        tk.Label(f, text=title, font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 15))
        lbl_val = tk.Label(f, text=val_text, font=('Inter', 32, 'bold'), fg=PAL['accent'], bg=PAL['panel'])
        lbl_val.pack(anchor='w', pady=5)
        pbar = ttk.Progressbar(f, style='TPB.Horizontal.TProgressbar', length=300, mode='determinate')
        pbar.pack(anchor='w', pady=10)
        tk.Label(f, text=f'Active Monitoring: {desc}', font=('Inter', 8, 'italic'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=5)
        f.val_lbl = lbl_val
        f.pbar = pbar
        return f