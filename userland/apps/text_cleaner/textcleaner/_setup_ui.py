"""
Auto-split from userland\apps\text_cleaner.py — TextCleaner._setup_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random



class TextCleaner:
    def _setup_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        head.pack(side='top', fill='x')
        tk.Label(head, text='TEXTCLEANER PRO', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.stats = tk.Label(head, text='0 CHARS | 0 WORDS | RAW', font=('Inter', 10), fg=PAL['dim'], bg=PAL['bg'])
        self.stats.pack(side='right', pady=10)
        body = tk.Frame(self, bg=PAL['bg'], padx=40)
        body.pack(fill='both', expand=True)
        self.ctrl_fr = tk.Frame(body, bg=PAL['sidebar'], width=300, padx=20, pady=20)
        self.ctrl_fr.pack(side='left', fill='y', padx=(0, 20))
        self.ctrl_fr.pack_propagate(False)
        tk.Label(self.ctrl_fr, text='LOGIC CONFIG', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w', pady=(0, 15))
        sections = {'⚡ WHITESPACE': [('Trim Edges', 'trim', True), ('Collapse Spaces', 'coll', True), ('Purge Empty', 'purge', True)], '🛡️ SECURITY': [('Redact PII', 'pii', False), ('Strip HTML', 'html', False), ('Clean URLs', 'url', False)], '🔠 CASE': [('Sentence Case', 'sent', False), ('Lower Case', 'low', False), ('Upper Case', 'up', False)]}
        for name, opts in sections.items():
            tk.Label(self.ctrl_fr, text=name, font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['sidebar'], pady=10).pack(anchor='w')
            for lbl, key, dflt in opts:
                v = tk.BooleanVar(value=dflt)
                self.vars[key] = v
                tk.Checkbutton(self.ctrl_fr, text=lbl, variable=v, bg=PAL['sidebar'], fg=PAL['text'], selectcolor='#000', activebackground=PAL['sidebar'], font=('Inter', 9)).pack(anchor='w', padx=10)
        self.work_fr = tk.Frame(body, bg=PAL['bg'])
        self.work_fr.pack(side='right', fill='both', expand=True)
        self.tabs = ttk.Notebook(self.work_fr)
        self.tabs.pack(fill='both', expand=True)
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[20, 10], font=('Inter', 9, 'bold'))
        style.map('TNotebook.Tab', background=[('selected', PAL['accent'])])
        self.in_txt = scrolledtext.ScrolledText(self.tabs, bg='#000', fg=PAL['text'], font=('JetBrains Mono', 11), borderwidth=0, padx=20, pady=20)
        self.tabs.add(self.in_txt, text=' [ INPUT BUFFER ] ')
        self.in_txt.insert('1.0', '[PASTE RAW TEXT TO NORMALIZE]')
        self.in_txt.bind('<<Modified>>', self._update_stats)
        self.out_txt = scrolledtext.ScrolledText(self.tabs, bg='#000', fg=PAL['success'], font=('JetBrains Mono', 11), borderwidth=0, padx=20, pady=20)
        self.tabs.add(self.out_txt, text=' [ OUTPUT RESULT ] ')
        foot = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        foot.pack(side='bottom', fill='x')
        tk.Button(foot, text='TRIGGER NEURAL CLEAN', font=('Inter', 11, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=35, pady=10, command=self._process).pack(side='right')
        tk.Button(foot, text='COPY BUFFER', font=('Inter', 10, 'bold'), bg=PAL['sidebar'], fg=PAL['text'], relief='flat', padx=25, pady=10, command=self._copy).pack(side='right', padx=15)
        tk.Button(foot, text='✨ AI ANALYZE', font=('Inter', 10, 'bold'), bg=PAL['sidebar'], fg=PAL['secondary'], relief='flat', padx=25, pady=10, command=self._analyze).pack(side='right')
        self.status = tk.Label(self, text='', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
