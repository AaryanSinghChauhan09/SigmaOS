# Generated method: AIStudio._build_ds_tab
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading
import sys
import os
from typing import Dict, Any, List, Optional

class AIStudio:
    def _build_ds_tab(self, parent):
        self.tabs.add(parent, text=f" {ICONS.get('viz_engine', '📊')} VECTORS ")
        tk.Label(parent, text='FEATURE ENGINEERING & EDA', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        cols_fr = tk.Frame(parent, bg=PAL['bg'])
        cols_fr.pack(fill='both', expand=True)
        ops = [('Dimensionality Reduction', 'PCA, t-SNE, UMAP computed natively on GPU.', '#00E0FF'), ('Automated Imputation', 'Missing values interpolated via recurrent neural guessing.', '#9D4EDD'), ('Statistical Outlier Purge', 'Z-Score & IQR bounded isolation forests.', '#FFD60A')]
        for name, desc, col in ops:
            f = tk.Frame(cols_fr, bg=PAL['panel'], pady=15, padx=20)
            f.pack(fill='x', pady=5)
            tk.Label(f, text=name, font=('Inter', 11, 'bold'), fg=col, bg=PAL['panel']).pack(anchor='w')
            tk.Label(f, text=desc, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(5, 0))
            tk.Button(f, text='APPLY', bg=PAL['sidebar'], fg='white', font=('Inter', 8, 'bold'), relief='flat', command=lambda n=name: self._apply_ds(n)).pack(side='right', pady=5)