"""
Auto-split from userland\apps\shell_forge.py — ShellForge._build_alias_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class ShellForge:
    def _build_alias_tab(self):
        tk.Label(self.tab_alias, text='ALIAS & FUNCTION MATRIX', font=('Inter', 13, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 15))
        aliases = [('ll', 'ls -la --color=auto'), ('gs', 'git status'), ('gc', 'git commit -m'), ('purge', 'sudo omni_purge --deep'), ('sigma', 'cd ~/SigmaOS && source .env'), ('update', 'sudo package_weaver --sync && --upgrade-all')]
        self.alias_text = tk.Text(self.tab_alias, bg=PAL['panel'], fg=PAL['text'], font=('Consolas', 11), relief='flat', height=14)
        self.alias_text.pack(fill='both', expand=True, pady=(0, 15))
        for alias, cmd in aliases:
            self.alias_text.insert(tk.END, f"alias {alias}='{cmd}'\n")
        btn_fr = tk.Frame(self.tab_alias, bg=PAL['bg'])
        btn_fr.pack(fill='x')
        tk.Button(btn_fr, text='💾 SAVE TO .ZSHRC', font=('Inter', 9, 'bold'), bg=PAL['accent_dim'], fg='white', relief='flat', padx=15, pady=8, command=lambda: messagebox.showinfo('Saved', 'Aliases written to ~/.zshrc\nSource reloaded.')).pack(side='left')
        tk.Button(btn_fr, text='➕ ADD ALIAS', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=lambda: self.alias_text.insert(tk.END, "alias newcmd=''\n")).pack(side='left', padx=10)
