"""
Auto-split from userland\apps\ncert_master_lab.py — NCERTMasterLab._build_form
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import importlib, sys, os, traceback, json, time
from typing import Dict, Any, List, Optional



class NCERTMasterLab:
    def _build_form(self, cls, name, data, color):
        for w in self._mid.winfo_children():
            w.destroy()
        method_name, fields = data
        tk.Label(self._mid, text=name, fg=color, bg=PAL['bg'], font=('Segoe UI Bold', 16)).pack(pady=20)
        entries = {}
        for f_label, f_def in fields:
            row = tk.Frame(self._mid, bg=PAL['bg'])
            row.pack(fill='x', padx=25, pady=6)
            tk.Label(row, text=f_label, fg=PAL['text'], bg=PAL['bg'], width=20, anchor='w').pack(side='left')
            e = tk.Entry(row, bg=PAL['card'], fg='white', relief='flat')
            e.insert(0, str(f_def))
            e.pack(side='right', fill='x', expand=True)
            entries[f_label] = e

        def run_sim():
            try:
                args = [float(entries[l].get()) if '.' in entries[l].get() else int(entries[l].get()) for l, _ in fields]
                res = getattr(cls, method_name)(*args)
                self._completed_count += 1
                if self.engine and hasattr(self.engine, 'earn_xp'):
                    self.engine.earn_xp(150)
                self._show_res(name, res)
            except Exception:
                self._out.insert('end', traceback.format_exc(), 'err')
        tk.Button(self._mid, text=f"{ICONS.get('bootloader', '🚀')} EXECUTE SIMULATION", bg=color, fg='white', command=run_sim, pady=10).pack(fill='x', padx=25, pady=35)
