"""
Auto-split from userland\apps\ncert_master_lab.py — NCERTMasterLab._build_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional



class NCERTMasterLab:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['panel'], height=65)
        hdr.pack(fill='x')
        hdr.pack_propagate(False)
        tk.Label(hdr, text=f"{ICONS.get('ncert', '🔬')} LAB v10.0", fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 18)).pack(side='left', padx=25)
        self._status_lbl = tk.Label(hdr, text='[SYSTEM OPERATIONAL]', fg='#00D26A', bg=PAL['panel'], font=('Consolas', 9))
        self._status_lbl.pack(side='right', padx=25)
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=12, pady=12)
        self._tree_fr = tk.Frame(body, bg=PAL['panel'], width=320)
        self._tree_fr.pack(side='left', fill='y', padx=(0, 12))
        self._tree_fr.pack_propagate(False)
        self.search_ent = tk.Entry(self._tree_fr, bg=PAL['card'], fg='white', font=('Segoe UI', 9), relief='flat')
        self.search_ent.pack(fill='x', padx=10, pady=10)
        self.search_ent.insert(0, 'Filter experiments...')
        self._tree = ttk.Treeview(self._tree_fr, show='tree', selectmode='browse')
        self._tree.pack(fill='both', expand=True)
        self._tree.bind('<<TreeviewSelect>>', self._on_select)
        self._mid = tk.Frame(body, bg=PAL['bg'], width=450)
        self._mid.pack(side='left', fill='y', padx=(0, 12))
        self._mid.pack_propagate(False)
        self._mid_msg = tk.Label(self._mid, text='◄ SELECT AN EXPERIMENT', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI Bold', 12))
        self._mid_msg.pack(expand=True)
        self._out_fr = tk.Frame(body, bg=PAL['bg'])
        self._out_fr.pack(side='right', fill='both', expand=True)
        self._out = scrolledtext.ScrolledText(self._out_fr, bg='#070910', fg='#00D26A', font=('Cascadia Code', 10), borderwidth=0, padx=20, pady=20)
        self._out.pack(fill='both', expand=True)
        self._out.tag_config('title', foreground=PAL['accent'], font=('Segoe UI Bold', 15))
        self._out.tag_config('badge', foreground=PAL['ch'], font=('Segoe UI Bold', 12))
        self._out.tag_config('err', foreground='#FF4B4B')
