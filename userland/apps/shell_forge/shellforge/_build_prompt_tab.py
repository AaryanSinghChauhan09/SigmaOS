"""
Auto-split from userland\apps\shell_forge.py — ShellForge._build_prompt_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _build_prompt_tab(self):
        tk.Label(self.tab_prompt, text='POWERLEVEL10K PROMPT ARCHITECT', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        segments = [('OS Icon', True, '#BD00FF'), ('Username', True, '#007AFF'), ('Directory (truncated 3)', True, '#00FF88'), ('Git Branch', True, '#FFD60A'), ('Virtual Env', False, '#FF007F'), ('Execution Time', True, '#FF3B30'), ('Battery Status', False, '#32D74B'), ('Background Jobs', True, '#00FFCC')]
        tk.Label(self.tab_prompt, text='PROMPT SEGMENTS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        seg_fr = tk.Frame(self.tab_prompt, bg=PAL['bg'])
        seg_fr.pack(fill='x')
        for name, default, col in segments:
            f = tk.Frame(seg_fr, bg=PAL['panel'], padx=15, pady=10)
            f.pack(fill='x', pady=4)
            var = tk.BooleanVar(value=default)
            cb = tk.Checkbutton(f, variable=var, bg=PAL['panel'], selectcolor=PAL['sidebar'], activebackground=PAL['panel'])
            cb.pack(side='left')
            swatch = tk.Label(f, bg=col, width=3)
            swatch.pack(side='left', padx=8)
            tk.Label(f, text=name, font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(side='left')
        tk.Label(self.tab_prompt, text='LIVE PREVIEW', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(20, 5))
        preview = tk.Frame(self.tab_prompt, bg='#060608', padx=10, pady=10)
        preview.pack(fill='x')
        segments_preview = [('  ', '#BD00FF'), ('sovereign ', '#007AFF'), ('~/SigmaOS/userland ', '#00FF88'), ('git:master ', '#FFD60A'), ('0.42s ', '#FF3B30'), ('❯ ', '#F2F2F7')]
        for seg, col in segments_preview:
            tk.Label(preview, text=seg, bg='#060608', fg=col, font=('JetBrains Mono', 12, 'bold')).pack(side='left')
