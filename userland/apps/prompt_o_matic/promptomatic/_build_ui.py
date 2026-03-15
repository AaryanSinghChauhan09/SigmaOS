# Generated method: PromptOMatic._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import webbrowser
import urllib.parse
import time

class PromptOMatic:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=30, pady=25)
        head.pack(fill='x')
        tk.Label(head, text='🔮 AI COMMAND CENTER', font=('Inter', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        grid_fr = tk.LabelFrame(self, text=' TARGET NEURAL NODES ', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg'], padx=20, pady=20, highlightthickness=1, highlightbackground=PAL['border'])
        grid_fr.pack(fill='x', padx=30, pady=10)
        for i, (name, var) in enumerate(self.targets.items()):
            cb = tk.Checkbutton(grid_fr, text=name, variable=var, bg=PAL['bg'], fg='white', selectcolor='#000', activebackground=PAL['bg'], activeforeground=PAL['accent'], font=('Inter', 10))
            cb.grid(row=i // 3, column=i % 3, sticky='w', padx=30, pady=5)
        lab_fr = tk.Frame(self, bg=PAL['bg'], padx=30, pady=10)
        lab_fr.pack(fill='both', expand=True)
        tk.Label(lab_fr, text='PROMPT BLUEPRINT', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w')
        self.txt = tk.Text(lab_fr, bg='#000', fg=PAL['text'], font=('JetBrains Mono', 11), insertbackground='white', borderwidth=0, padx=20, pady=20, highlightthickness=1, highlightbackground=PAL['border'])
        self.txt.pack(fill='both', expand=True, pady=10)
        self.txt.insert('1.0', 'Analyze the sovereign architecture of SigmaOS vs legacy platforms.')
        foot = tk.Frame(self, bg=PAL['bg'], padx=30, pady=25)
        foot.pack(fill='x')
        self.auto_login = tk.BooleanVar(value=True)
        tk.Checkbutton(foot, text='INJECT SOVEREIGN CREDENTIALS (AUTO-LOGIN)', variable=self.auto_login, bg=PAL['bg'], fg=PAL['success'], font=('Inter', 8, 'bold')).pack(side='left')
        tk.Button(foot, text='🚀 DISPATCH VECTORS', font=('Inter', 11, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=40, pady=12, command=self.dispatch).pack(side='right')