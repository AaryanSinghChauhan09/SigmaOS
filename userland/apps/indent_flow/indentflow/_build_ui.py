# Generated method: IndentFlow._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random

class IndentFlow:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=30, pady=25)
        head.pack(fill='x')
        tk.Label(head, text='INDENTFLOW PRO', font=('Inter', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(head, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tools = [('🗺️ GENERATE MAP', self._render), ('💾 EXPORT', self._export), ('⚡ ANALYZE', self._analyze)]
        for txt, cmd in tools:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        body = tk.Frame(self, bg=PAL['bg'], padx=25)
        body.pack(fill='both', expand=True)
        self.panes = ttk.PanedWindow(body, orient='horizontal')
        self.panes.pack(fill='both', expand=True)
        self.code_fr = tk.Frame(self.panes, bg=PAL['panel'], width=500, padx=15, pady=20)
        self.panes.add(self.code_fr, weight=2)
        tk.Label(self.code_fr, text='SOURCE CODE (LOCAL/VAULT)', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.txt = scrolledtext.ScrolledText(self.code_fr, bg='#000', fg=PAL['text'], font=('JetBrains Mono', 10), insertbackground='white', borderwidth=0, padx=15, pady=15, undo=True)
        self.txt.pack(fill='both', expand=True, pady=10)
        self.txt.insert('1.0', 'def handle_authentication(user_key):\n    if user_key.is_valid():\n        dispatch_session()\n        for service in mesh_registry:\n            service.sync()\n            if service.critical:\n                wait_for_ack()\n    return True')
        self.map_fr = tk.Frame(self.panes, bg=PAL['bg'], padx=15, pady=20)
        self.panes.add(self.map_fr, weight=3)
        tk.Label(self.map_fr, text='LOGIC RECONSTRUCTION (NEURAL MAPPING)', font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w')
        self.canv = tk.Canvas(self.map_fr, bg=PAL['bg'], highlightthickness=1, highlightbackground=PAL['border'])
        self.canv.pack(fill='both', expand=True, pady=10)
        self.status = tk.Label(self, text='INDENTFLOW ENGINE READY | LOGIC ADAPTER: PYTHON_3 | SYMBOL_TABLE: LOADED', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')