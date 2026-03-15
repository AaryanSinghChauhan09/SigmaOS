# Generated method: SpaceExplorer._build_ui
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import os
import random

class SpaceExplorer:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=60, padx=20)
        self.header.pack(side='top', fill='x', pady=10)
        tk.Label(self.header, text='SPACE EXPLORER APEX', font=('Inter', 18, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('🌐 TARGET', self._select_target), ('⚡ QUICK SCAN', self._quick_scan), ('🧬 DEEP NEURAL SCAN', self._deep_scan), ('🧹 PURGE', self._purge_junk)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=12, pady=6, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=20, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.left_panel = tk.Frame(self.workspace, bg=PAL['panel'], width=250, padx=15, pady=15)
        self.left_panel.pack(side='left', fill='y')
        self.left_panel.pack_propagate(False)
        tk.Label(self.left_panel, text='DRIVE TELEMETRY', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        self.drive_lbl = tk.Label(self.left_panel, text=f'TARGET: {self.target_drive}', font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['panel'])
        self.drive_lbl.pack(anchor='w', pady=5)
        self.prog_lbl = tk.Label(self.left_panel, text='STORAGE CAPACITY', font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel'])
        self.prog_lbl.pack(anchor='w', pady=(15, 5))
        self.pbar = ttk.Progressbar(self.left_panel, style='TProgressbar', length=220, mode='determinate')
        self.pbar.pack(anchor='w', pady=5)
        self.pbar['value'] = 65
        tk.Label(self.left_panel, text='65% USED  |  35% FREE', font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(anchor='w', pady=5)
        self.visualizer = tk.Frame(self.workspace, bg=PAL['bg'], padx=15)
        self.visualizer.pack(side='left', fill='both', expand=True)
        tk.Label(self.visualizer, text='NEURAL DATA MAP', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.canvas = tk.Canvas(self.visualizer, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, pady=10)
        self._draw_mock_map()
        self.right_panel = tk.Frame(self.workspace, bg=PAL['panel'], width=220, padx=15, pady=15)
        self.right_panel.pack(side='right', fill='y', padx=(15, 0))
        self.right_panel.pack_propagate(False)
        tk.Label(self.right_panel, text='VECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        vectors = [('SYSTEM', '24.5 GB', PAL['accent']), ('APPS', '112.1 GB', PAL['success']), ('MEDIA', '48.2 GB', '#FFA500'), ('JUNK', '4.1 GB', PAL['danger'])]
        for name, size, color in vectors:
            f = tk.Frame(self.right_panel, bg=PAL['panel'], pady=8)
            f.pack(fill='x')
            tk.Label(f, text=name, font=('Inter', 8, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(side='left')
            tk.Label(f, text=size, font=('Inter', 9, 'bold'), fg=color, bg=PAL['panel']).pack(side='right')
        self.status = tk.Label(self, text='SOVEREIGN SPACE [V4.0] | IDLE', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')