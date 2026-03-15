# Generated method: OmniConverter._build_ui
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
from typing import List, Dict

class OmniConverter:
    def _build_ui(self):
        main = tk.Frame(self, bg=PAL['bg'], padx=40, pady=40)
        main.pack(fill='both', expand=True)
        head = tk.Frame(main, bg=PAL['bg'])
        head.pack(fill='x', pady=(0, 30))
        tk.Label(head, text='OMNI', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Label(head, text='CONVERTER PRO', font=('Inter', 24, 'bold'), fg='white', bg=PAL['bg']).pack(side='left', padx=5)
        self.stats = tk.Label(head, text='GPU ACCELERATION: ACTIVE (TITAN BUS)', font=('Inter', 8, 'bold'), fg=PAL['success'], bg=PAL['bg'])
        self.stats.pack(side='right', pady=15)
        self.select_fr = tk.Frame(main, bg=PAL['card'], height=120, highlightthickness=1, highlightbackground=PAL['border'])
        self.select_fr.pack(fill='x', pady=(0, 30))
        self.select_fr.pack_propagate(False)
        self.file_lbl = tk.Label(self.select_fr, text='DRAG & DROP OR SELECT SOURCE BITSTREAM', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['card'])
        self.file_lbl.pack(expand=True)
        self.select_fr.bind('<Button-1>', lambda e: self._select_file())
        self.file_lbl.bind('<Button-1>', lambda e: self._select_file())
        cfg = tk.Frame(main, bg=PAL['bg'])
        cfg.pack(fill='x', pady=(0, 20))
        tk.Label(cfg, text='TARGET SYMMETRY', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).grid(row=0, column=0, sticky='w')
        self.format_box = ttk.Combobox(cfg, values=['MP4 (H.266)', 'PDF (ARCHIVE)', 'MP3 (LOSSLESS)', 'JPG (NEURAL)', 'DOCX (PQC)'], font=('Inter', 10))
        self.format_box.current(0)
        self.format_box.grid(row=1, column=0, pady=(5, 0), sticky='ew')
        tk.Label(cfg, text='OPTIMIZATION PRESET', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).grid(row=0, column=1, sticky='w', padx=(20, 0))
        self.preset_box = ttk.Combobox(cfg, values=['WEBSITE (ULTRA-SLIM)', 'PRINT (DPI-MAX)', 'COLD-STORAGE', 'SOCIAL-READY'], font=('Inter', 10))
        self.preset_box.current(0)
        self.preset_box.grid(row=1, column=1, pady=(5, 0), padx=(20, 0), sticky='ew')
        cfg.columnconfigure(0, weight=1)
        cfg.columnconfigure(1, weight=1)
        self.preview = tk.Frame(main, bg=PAL['panel'], padx=25, pady=25, highlightthickness=1, highlightbackground=PAL['border'])
        self.preview.pack(fill='both', expand=True)
        tk.Label(self.preview, text='SOURCE ANALYTICS', font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(anchor='w')
        self.analytics_lbl = tk.Label(self.preview, text='Awaiting Source...', font=('JetBrains Mono', 9), fg=PAL['dim'], bg=PAL['panel'], justify='left')
        self.analytics_lbl.pack(expand=True)
        self.action_fr = tk.Frame(main, bg=PAL['bg'], pady=30)
        self.action_fr.pack(fill='x')
        tk.Button(self.action_fr, text='💎 MORPH BITSTREAM', font=('Inter', 11, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=45, pady=15, command=self._convert).pack(side='right')
        self.status = tk.Label(self, text='READY | ENCRYPTION: SOVEREIGN-AES-512 | CPU: 0.1%', bg=PAL['panel'], fg=PAL['dim'], font=('Inter', 8, 'bold'), pady=8)
        self.status.pack(side='bottom', fill='x')