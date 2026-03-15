"""
Auto-split from userland\apps\sigma_antigravity.py — SigmaAntigravity._refresh_quota_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional



class SigmaAntigravity:
    def _refresh_quota_ui(self):
        for w in self.quota_fr.winfo_children():
            w.destroy()
        for name, data in self.engine.quotas.items():
            card = tk.Frame(self.quota_fr, bg=PAL['card'], padx=15, pady=10, highlightthickness=1, highlightbackground=PAL['border'])
            card.pack(fill='x', pady=5)
            tk.Label(card, text=name, font=('Inter Bold', 10), fg=PAL['accent'], bg=PAL['card']).pack(side='left')
            tk.Label(card, text=f"{data['used']} / {data['limit']} {data['unit']}", font=('Inter', 9), fg=PAL['text'], bg=PAL['card']).pack(side='right')
