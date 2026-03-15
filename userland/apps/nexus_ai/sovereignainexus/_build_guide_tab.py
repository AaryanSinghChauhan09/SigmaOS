"""
Auto-split from userland\apps\nexus_ai.py — SovereignAINexus._build_guide_tab
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json



class SovereignAINexus:
    def _build_guide_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'], padx=15, pady=15)
        self.nb.add(tab, text='  📖 OS Guide  ')
        left = tk.Frame(tab, bg=PAL['sidebar'], width=250)
        left.pack(side='left', fill='y', padx=(0, 15))
        left.pack_propagate(False)
        tk.Label(left, text='MODULES & PATHS', font=('Segoe UI', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar'], pady=10).pack()
        guides = {'Introduction': 'Welcome to SigmaOS. The Zero-Trust, Neuro-Native environment.', 'Security Guardian (Sentinel)': "Sentinel is your 5-tab security center. Use 'Hex-Scan' for deep validation.", 'CodeForge IDE': 'A professional-grade IDE. Supports real Python code execution and sandbox terminal.', 'Antigravity AI Hub': 'Orchestrate 13+ AI platforms from a single point. Manage quotas and presets.', 'Sovereign Writer': 'A minimalist, privacy-focused text editor for secure document authoring.', 'PulsePlayer': 'Music with neural upsampling and integrated EQ. Bit-perfect playback paths.', 'File Explorer (VFS)': 'The Virtual File System mirrors your project root with AI-driven cleanup.', 'Automation Hub': "Tasker/Shortcuts parity. Automate everything with 'Shortcut Forge'.", 'OmniBrowser': 'The multi-engine, sandboxed browser core for absolute web privacy.'}
        self.guide_text = scrolledtext.ScrolledText(tab, bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 10), borderwidth=0, padx=20, pady=20)
        self.guide_text.pack(side='right', fill='both', expand=True)

        def _load_guide(title):
            self.guide_text.delete('1.0', 'end')
            self.guide_text.insert('end', f'{title.upper()}\n', 'title')
            self.guide_text.insert('end', '=' * len(title) + '\n\n', 'title')
            self.guide_text.insert('end', guides[title])
            self.guide_text.tag_config('title', foreground=PAL['accent'], font=('Segoe UI Bold', 14))
        for g in guides:
            btn = tk.Button(left, text=g, font=('Segoe UI', 9), bg=PAL['sidebar'], fg=PAL['text'], relief='flat', anchor='w', padx=10, pady=5, command=lambda t=g: _load_guide(t))
            btn.pack(fill='x')
            btn.bind('<Enter>', lambda e, b=btn: b.config(bg=PAL['card']))
            btn.bind('<Leave>', lambda e, b=btn: b.config(bg=PAL['sidebar']))
        _load_guide('Introduction')
