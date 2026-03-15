# Generated method: VortexClipboard._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import time

class VortexClipboard:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='VORTEX CLIPBOARD', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🧹 PURGE ALL MULTIVERSE DATA', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', padx=15, pady=8, command=self._purge_clips).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.conf_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=200, padx=15, pady=20)
        self.conf_fr.pack(side='left', fill='y', padx=(0, 20))
        self.conf_fr.pack_propagate(False)
        tk.Label(self.conf_fr, text='TEMPORAL SETTINGS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        opts = [('Neural Sync', True), ('Zero-Day Retain', False), ('Crypto Lock', True)]
        for text, state in opts:
            var = tk.BooleanVar(value=state)
            cb = tk.Checkbutton(self.conf_fr, text=text, variable=var, bg=PAL['panel'], fg=PAL['text'], selectcolor=PAL['sidebar'], activebackground=PAL['panel'], activeforeground=PAL['accent'], font=('Inter', 9))
            cb.pack(anchor='w', pady=5)
        self.tree_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.tree_fr.pack(side='left', fill='both', expand=True)
        cols = ('Type', 'Content Data / Hash', 'Temporal Stamp')
        self.tree = ttk.Treeview(self.tree_fr, columns=cols, show='headings', style='Clip.Treeview', height=15)
        self.tree.heading('Type', text='VECTOR')
        self.tree.column('Type', width=80, anchor='center')
        self.tree.heading('Content Data / Hash', text='DATA PAYLOAD')
        self.tree.column('Content Data / Hash', width=350)
        self.tree.heading('Temporal Stamp', text='TEMPORAL STAMP')
        self.tree.column('Temporal Stamp', width=120, anchor='center')
        for item in self.history:
            self.tree.insert('', 'end', values=item)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._inject_clip)
        self.status = tk.Label(self, text='VORTEX LISTENING | AES-512 SECURED MEMORY', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')