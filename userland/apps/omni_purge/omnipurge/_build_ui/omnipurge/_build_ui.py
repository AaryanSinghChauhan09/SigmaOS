# Generated method: OmniPurge._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniPurge:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='OMNI-PURGE ENGINE', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🚀 INITIATE PURGE', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', padx=15, pady=8, command=self._start_purge).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.vec_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=400, padx=20, pady=20)
        self.vec_fr.pack(side='left', fill='y', padx=(0, 20))
        self.vec_fr.pack_propagate(False)
        tk.Label(self.vec_fr, text='TARGET PURGE VECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 20))
        for text, var in self.categories.items():
            cb = tk.Checkbutton(self.vec_fr, text=text, variable=var, bg=PAL['panel'], fg=PAL['text'], selectcolor=PAL['sidebar'], activebackground=PAL['panel'], activeforeground=PAL['accent'], font=('Inter', 9), anchor='w', justify='left')
            cb.pack(fill='x', pady=5)
        tk.Button(self.vec_fr, text='🔍 ANALYZE SELECTED VECTORS', font=('Inter', 8, 'bold'), bg=PAL['accent'], fg='black', relief='flat', pady=8, command=self._analyze_space).pack(fill='x', pady=(20, 0))
        self.out_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.out_fr.pack(side='left', fill='both', expand=True)
        self.mass_lbl = tk.Label(self.out_fr, text='0.00 GB', font=('Inter', 48, 'bold'), fg=PAL['text'], bg=PAL['bg'])
        self.mass_lbl.pack(pady=(20, 0))
        tk.Label(self.out_fr, text='Total Mass Slated for Obliteration', font=('Inter', 10), fg=PAL['dim'], bg=PAL['bg']).pack()
        self.term = tk.Text(self.out_fr, bg=PAL['panel'], fg=PAL['success'], font=('Consolas', 10), relief='flat')
        self.term.pack(fill='both', expand=True, pady=20, padx=20)
        self.term.insert(tk.END, '>>> [OMNI-PURGE MODULE LOADED]\n')
        self.term.config(state=tk.DISABLED)
        self.pbar = ttk.Progressbar(self.workspace, style='Purge.Horizontal.TProgressbar', length=100, mode='determinate')
        self.status = tk.Label(self, text='IDLE | AWAITING COMMAND AUTHORIZATION', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')