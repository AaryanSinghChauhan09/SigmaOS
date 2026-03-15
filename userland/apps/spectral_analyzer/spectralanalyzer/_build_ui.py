# Generated method: SpectralAnalyzer._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random

class SpectralAnalyzer:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='SPECTRAL DISK ARRAY', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='📊 INITIATE DEEP SCAN', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=15, pady=8, command=self._start_scan).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.conf_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=20, pady=20)
        self.conf_fr.pack(side='left', fill='y', padx=(0, 20))
        self.conf_fr.pack_propagate(False)
        tk.Label(self.conf_fr, text='CAPACITY VECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        tk.Label(self.conf_fr, text='120.4 GB / 256.0 GB USED', font=('Inter', 12, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w', pady=5)
        metrics = [('Media Arrays', '45 GB', '#1E90FF'), ('Compiled Code', '20 GB', '#00FA9A'), ('System Blobs', '15 GB', '#FF6347'), ('Encrypted Keys', '5 GB', '#9370DB')]
        for label, val, color in metrics:
            row = tk.Frame(self.conf_fr, bg=PAL['panel'], pady=5)
            row.pack(fill='x')
            tk.Label(row, text='●', font=('Inter', 12), fg=color, bg=PAL['panel']).pack(side='left')
            tk.Label(row, text=label, font=('Inter', 9, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(side='left', padx=5)
            tk.Label(row, text=val, font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(side='right')
        self.viz_fr = tk.Frame(self.workspace, bg=PAL['panel'], padx=15, pady=15)
        self.viz_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.viz_fr, text='NEURAL HEURISTIC TOPOGRAPHY (TREEMAP)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        self.canvas = tk.Canvas(self.viz_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True)
        self._draw_mock_treemap()
        self.status = tk.Label(self, text='SPECTRAL INDEXING DORMANT | WAITING FOR TARGET', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')