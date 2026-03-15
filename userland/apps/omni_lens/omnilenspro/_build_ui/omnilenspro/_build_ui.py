# Generated method: OmniLensPro._build_ui
import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading

class OmniLensPro:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-LENS VISION', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('📷 FEED', self._simulate_camera), ('📁 FILE', self._select_image)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        tk.Button(btn_fr, text='🧠 NEURAL PARSE', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=15, pady=8, command=self._start_scan).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.view_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=500, padx=10, pady=10)
        self.view_fr.pack(side='left', fill='both', expand=True, padx=(0, 10))
        self.view_fr.pack_propagate(False)
        tk.Label(self.view_fr, text='OPTICAL VIEWPORT', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.canvas = tk.Canvas(self.view_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, pady=10)
        self.canvas.create_text(250, 250, text='AWAITING VISUAL FEED...', fill=PAL['dim'], font=('Inter', 12, 'bold'))
        self.scan_line = None
        self.res_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=400, padx=15, pady=15)
        self.res_fr.pack(side='left', fill='both', expand=True)
        self.res_fr.pack_propagate(False)
        tk.Label(self.res_fr, text='EXTRACTED VECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.res_text = tk.Text(self.res_fr, bg=PAL['sidebar'], fg=PAL['success'], font=('Consolas', 11), relief='flat')
        self.res_text.pack(fill='both', expand=True, pady=10)
        self.res_text.insert(tk.END, '>>> READY FOR INGEST.\n')
        self.res_text.config(state=tk.DISABLED)
        self.status = tk.Label(self, text='VISION SENSORS DORMANT | ON-DEVICE ML LOADED', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')