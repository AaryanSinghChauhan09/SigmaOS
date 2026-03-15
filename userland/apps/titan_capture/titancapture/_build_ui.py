# Generated method: TitanCapture._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class TitanCapture:
    def _build_ui(self):
        main = tk.Frame(self, bg=PAL['bg'], padx=30, pady=30)
        main.pack(fill='both', expand=True)
        head = tk.Frame(main, bg=PAL['bg'])
        head.pack(fill='x', pady=(0, 25))
        tk.Label(head, text='TITAN', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Label(head, text='CAPTURE PRO', font=('Inter', 20, 'bold'), fg='white', bg=PAL['bg']).pack(side='left', padx=5)
        self.light = tk.Canvas(head, width=12, height=12, bg=PAL['bg'], highlightthickness=0)
        self.light.pack(side='right', pady=10)
        self.light.create_oval(2, 2, 10, 10, fill=PAL['dim'], tags='dot')
        cfg = tk.Frame(main, bg=PAL['card'], padx=20, pady=20, highlightthickness=1, highlightbackground=PAL['border'])
        cfg.pack(fill='x')
        tk.Label(cfg, text='STREAM QUALITY', font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['card']).grid(row=0, column=0, sticky='w')
        self.qual = ttk.Combobox(cfg, values=['4K LOSSLESS', '1080P APEX', '720P MOBILE'], style='Titan.TCombobox')
        self.qual.current(0)
        self.qual.grid(row=1, column=0, sticky='ew', pady=(5, 15))
        tk.Label(cfg, text='TARGET FPS', font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['card']).grid(row=0, column=1, sticky='w', padx=(20, 0))
        self.fps = ttk.Combobox(cfg, values=['144 FPS', '120 FPS', '60 FPS'], width=10, style='Titan.TCombobox')
        self.fps.current(0)
        self.fps.grid(row=1, column=1, sticky='ew', pady=(5, 15), padx=(20, 0))
        tk.Label(cfg, text='WORKSPACE TARGET', font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['card']).grid(row=2, column=0, columnspan=2, sticky='w')
        self.area = ttk.Combobox(cfg, values=['Global Screen', 'Active Sandbox', 'Custom Grid'], style='Titan.TCombobox')
        self.area.current(1)
        self.area.grid(row=3, column=0, columnspan=2, sticky='ew', pady=(5, 0))
        cfg.columnconfigure(0, weight=1)
        cfg.columnconfigure(1, weight=1)
        self.timer = tk.Label(main, text='00:00:00', font=('JetBrains Mono', 18, 'bold'), fg='white', bg=PAL['bg'], pady=20)
        self.timer.pack()
        self.btn = tk.Button(main, text='● INITIATE CAPTURE', font=('Inter', 11, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=40, pady=15, command=self._toggle)
        self.btn.pack(fill='x')
        self.status = tk.Label(main, text='READY | ENCRYPTION: IDLE', font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['bg'], pady=10)
        self.status.pack()