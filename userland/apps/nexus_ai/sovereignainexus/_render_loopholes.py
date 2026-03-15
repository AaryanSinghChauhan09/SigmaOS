"""
Auto-split from userland\apps\nexus_ai.py — SovereignAINexus._render_loopholes
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json



class SovereignAINexus:
    def _render_loopholes(self):
        for w in self.lh_fr.winfo_children():
            w.destroy()
        if not self.loopholes:
            return
        for lh in self.loopholes.scan():
            card = tk.Frame(self.lh_fr, bg=PAL['card'], padx=15, pady=12, highlightthickness=1, highlightbackground=PAL['border'])
            card.pack(fill='x', pady=4)
            c1 = tk.Frame(card, bg=PAL['card'])
            c1.pack(side='left', fill='both', expand=True)
            status_col = PAL['danger'] if lh['status'] == 'DETECTED' else PAL['success']
            tk.Label(c1, text=f"• {lh['name']}", font=('Segoe UI Bold', 10), fg=status_col, bg=PAL['card']).pack(anchor='w')
            tk.Label(c1, text=lh['desc'], font=('Segoe UI', 9), fg=PAL['dim'], bg=PAL['card']).pack(anchor='w')
            if lh['status'] == 'DETECTED':
                btn = tk.Button(card, text='FIX LOOPHOLE', font=('Segoe UI', 8, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=15, command=lambda i=lh['id']: self._fix_lh(i))
                btn.pack(side='right')
            else:
                tk.Label(card, text='✓ MITIGATED', font=('Segoe UI', 8, 'bold'), fg=PAL['success'], bg=PAL['card']).pack(side='right')
