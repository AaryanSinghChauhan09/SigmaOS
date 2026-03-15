# Generated method: EchoCast._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='ECHO CAST PROTOCOL', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='📡 PING ETHER', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._start_scan).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.left_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=350, padx=20, pady=20)
        self.left_fr.pack(side='left', fill='y', padx=(0, 20))
        self.left_fr.pack_propagate(False)
        tk.Label(self.left_fr, text='TRANSMISSION METRICS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        metrics = [('Video Encoding:', 'H.266 Neural (8K Ready)'), ('Audio Protocol:', 'Quantum Lossless (192kHz)'), ('Target Latency:', 'Sub-8ms (Zero Frame Drop)'), ('Encryption:', 'Sovereign-AES-1024')]
        for k, v in metrics:
            tk.Label(self.left_fr, text=k, font=('Inter', 9), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(self.left_fr, text=v, font=('Inter', 10, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(anchor='w', pady=(2, 10))
        self.grid_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.grid_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.grid_fr, text='AVAILABLE RECEPTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.canvas = tk.Canvas(self.grid_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, pady=10)
        self.canvas.create_text(250, 150, text='📡 RADAR OFFLINE', fill=PAL['dim'], font=('Inter', 12, 'bold'))
        self.status = tk.Label(self, text='ECHO RECEIVER DORMANT | HARDWARE ENCODE READY', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')