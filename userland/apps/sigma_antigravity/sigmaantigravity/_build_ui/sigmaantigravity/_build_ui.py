# Generated method: SigmaAntigravity._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional

class SigmaAntigravity:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['header'], height=60)
        head.pack(fill='x')
        tk.Label(head, text='⚡ ANTIGRAVITY', font=('Inter Bold', 16), fg=PAL['accent'], bg=PAL['header']).pack(side='left', padx=20)
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill='both', expand=True)
        dist_fr = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(dist_fr, text='  🚀 Distributor  ')
        body = tk.Frame(dist_fr, bg=PAL['bg'], padx=20, pady=20)
        body.pack(fill='both', expand=True)
        left = tk.Frame(body, bg=PAL['panel'], width=250)
        left.pack(side='left', fill='y', padx=(0, 20))
        left.pack_propagate(False)
        tk.Label(left, text='AI FLEET', font=('Inter', 9, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(pady=10)
        for plat in self.engine.platforms:
            var = tk.BooleanVar(value=plat.get('tier', 2) == 1)
            self._sel_platforms[plat['name']] = var
            row = tk.Frame(left, bg=PAL['panel'])
            row.pack(fill='x', padx=10)
            tk.Checkbutton(row, variable=var, bg=PAL['panel']).pack(side='left')
            tk.Label(row, text=f"{plat.get('icon', '🤖')} {plat['name']}", fg=plat.get('color', 'white'), bg=PAL['panel']).pack(side='left')
        right = tk.Frame(body, bg=PAL['bg'])
        right.pack(side='left', fill='both', expand=True)
        self.prompt_txt = tk.Text(right, bg=PAL['card'], fg=PAL['text'], font=('Inter', 11), height=10, padx=15, pady=15)
        self.prompt_txt.pack(fill='x', pady=10)
        self.dispatch_btn = tk.Button(right, text='⚡ DISPATCH TO FLEET', bg=PAL['accent'], fg='white', font=('Inter Bold', 12), command=self._dispatch)
        self.dispatch_btn.pack(fill='x')
        self.log = scrolledtext.ScrolledText(right, bg='#050508', fg=PAL['green'], font=('Consolas', 9), height=12)
        self.log.pack(fill='both', expand=True, pady=(20, 0))
        self.quota_fr = tk.Frame(self.nb, bg=PAL['bg'], padx=20, pady=20)
        self.nb.add(self.quota_fr, text='  📊 Quotas  ')
        self._refresh_quota_ui()
        hist_fr = tk.Frame(self.nb, bg=PAL['bg'], padx=20, pady=20)
        self.nb.add(hist_fr, text='  📋 History  ')
        cols = ('Time', 'Platforms', 'Prompt Preview')
        self.hist_tree = ttk.Treeview(hist_fr, columns=cols, show='headings')
        for col in cols:
            self.hist_tree.heading(col, text=col)
        self.hist_tree.pack(fill='both', expand=True)