# Generated method: ProjectFlow._init_scrum
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _init_scrum(self, parent):
        cols = ['BACKLOG', 'IN PROGRESS', 'REVIEW', 'DONE']
        body = tk.Frame(parent, bg=PAL['bg'], pady=20)
        body.pack(fill='both', expand=True)
        for i, name in enumerate(cols):
            col_fr = tk.Frame(body, bg=PAL['sidebar'], width=280, highlightthickness=1, highlightbackground=PAL['border'])
            col_fr.pack(side='left', fill='both', expand=True, padx=10)
            col_fr.pack_propagate(False)
            tk.Label(col_fr, text=name, font=('Inter', 10, 'bold'), bg=PAL['sidebar'], fg=PAL['dim'], pady=15).pack(fill='x')
            for _ in range(random.randint(1, 3)):
                card = tk.Frame(col_fr, bg=PAL['card'], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL['border'])
                card.pack(fill='x', padx=10, pady=5)
                tk.Label(card, text=f'Task {random.randint(100, 999)}', font=('Inter', 10, 'bold'), bg=PAL['card'], fg='white').pack(anchor='w')
                tk.Label(card, text='Neural-Loom Optimization', font=('Inter', 8), bg=PAL['card'], fg=PAL['dim']).pack(anchor='w', pady=(5, 0))