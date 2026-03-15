"""
Auto-split from userland\apps\shell_forge.py — ShellForge._build_ui
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='SOVEREIGN SHELL FORGE (ZSH USURPER)', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='⚡ RELOAD PROFILE', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._reload_profile).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(self.workspace, style='Shell.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_term = tk.Frame(self.tabs, bg='#060608', padx=5, pady=5)
        self.tabs.add(self.tab_term, text='⚡ LIVE TERMINAL')
        self._build_terminal_tab()
        self.tab_prompt = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_prompt, text='💡 PROMPT ARCHITECT')
        self._build_prompt_tab()
        self.tab_plugins = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_plugins, text='🧩 PLUGIN ECOSYSTEM')
        self._build_plugin_tab()
        self.tab_alias = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_alias, text='🔗 ALIAS MATRIX')
        self._build_alias_tab()
        self.status = tk.Label(self, text='SHELL FORGE ONLINE | POWERLEVEL10K ENGINE READY | VI-MODE ENABLED', bg=PAL['accent_dim'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
