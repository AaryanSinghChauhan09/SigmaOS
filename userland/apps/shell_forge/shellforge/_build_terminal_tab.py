"""
Auto-split from userland\apps\shell_forge.py — ShellForge._build_terminal_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _build_terminal_tab(self):
        self.term_output = tk.Text(self.tab_term, bg='#060608', fg=PAL['text'], font=('JetBrains Mono', 10), relief='flat', insertbackground=PAL['accent'])
        self.term_output.pack(fill='both', expand=True, pady=(5, 0))
        self.term_output.insert(tk.END, 'Sovereign Shell v5.0 (zsh 5.9 compatible) - Neural Autocomplete ACTIVE\n')
        self.term_output.insert(tk.END, '──────────────────────────────────────────\n')
        self.term_output.config(state=tk.DISABLED)
        entry_fr = tk.Frame(self.tab_term, bg='#060608')
        entry_fr.pack(fill='x', pady=5)
        prompt_lbl = tk.Label(entry_fr, text='sovereign@apex ❯', fg=PAL['prompt'], bg='#060608', font=('JetBrains Mono', 11, 'bold'))
        prompt_lbl.pack(side='left', padx=(5, 8))
        self.cmd_entry = tk.Entry(entry_fr, font=('JetBrains Mono', 11), bg='#060608', fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.cmd_entry.pack(side='left', fill='x', expand=True)
        self.cmd_entry.bind('<Return>', self._exec_cmd)
        self.cmd_entry.focus()
