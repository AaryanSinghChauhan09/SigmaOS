"""
Auto-split from userland\apps\shell_forge.py — ShellForge._build_plugin_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _build_plugin_tab(self):
        tk.Label(self.tab_plugins, text='PLUGIN ECOSYSTEM (OMZ / Antigen Usurp)', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        plugins = [('zsh-autosuggestions', 'Fish-style inline command prediction from history.', True), ('zsh-syntax-highlighting', 'Syntax-aware colorization of typed commands.', True), ('fzf (Fuzzy Finder)', 'Ctrl+R reverse history via neural fuzzy search.', True), ('z (Autojump)', 'Frecency-based directory teleportation.', True), ('git-flow', 'Git branching model automations.', False), ('docker-sovereign', 'Aliases and completions for Quantum containers.', False)]
        for name, desc, enabled in plugins:
            f = tk.Frame(self.tab_plugins, bg=PAL['panel'], pady=12, padx=20)
            f.pack(fill='x', pady=5)
            var = tk.BooleanVar(value=enabled)
            tk.Checkbutton(f, variable=var, bg=PAL['panel'], selectcolor=PAL['sidebar'], activebackground=PAL['panel']).pack(side='left')
            tf = tk.Frame(f, bg=PAL['panel'])
            tf.pack(side='left', padx=10, fill='x', expand=True)
            tk.Label(tf, text=name, font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(tf, text=desc, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
