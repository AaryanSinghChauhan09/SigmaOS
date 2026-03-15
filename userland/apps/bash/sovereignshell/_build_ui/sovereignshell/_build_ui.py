# Generated method: SovereignShell._build_ui
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], height=30)
        head.pack(side='top', fill='x')
        tk.Label(head, text=' • sovereign_shell_01 ', font=('JetBrains Mono', 8), bg=PAL['border'], fg=PAL['text']).pack(side='left', padx=10, pady=5)
        self.terminal = scrolledtext.ScrolledText(self, bg=PAL['bg'], fg=PAL['text'], font=('JetBrains Mono', 11), borderwidth=0, insertbackground=PAL['prompt'], padx=20, pady=20)
        self.terminal.pack(fill='both', expand=True)
        self.terminal.bind('<Return>', self.handle_return)
        self.terminal.bind('<Up>', self.history_up)
        self.terminal.bind('<Down>', self.history_down)
        self.terminal.bind('<Tab>', self.handle_tab)
        self.status = tk.Label(self, text='SHELL: NEURAL-INGRESS ACTIVE | LATENCY: 0.2ms | INTEGRITY: 100%', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=4)
        self.status.pack(side='bottom', fill='x')