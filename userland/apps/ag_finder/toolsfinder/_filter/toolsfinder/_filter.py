# Generated method: ToolsFinder._filter
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import os
import time

class ToolsFinder:
    def _filter(self):
        q = self.search_var.get().lower() if self.search_var.get() != '[ NEURAL SEARCH ]' else ''
        for w in self.grid_fr.winfo_children():
            w.destroy()
        visible_tools = [t for t in self.tools if q in t.lower()]
        cols = 3
        for i, t in enumerate(visible_tools):
            r, c = (i // cols, i % cols)
            card = tk.Frame(self.grid_fr, bg=PAL['panel'], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL['border'])
            card.grid(row=r, column=c, padx=10, pady=10, sticky='nsew')
            icon = '🛠️' if 'Pro' not in t else '⚡'
            tk.Label(card, text=icon, font=('Inter', 24), bg=PAL['panel']).pack()
            tk.Label(card, text=t.upper(), font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(pady=(10, 0))
            tk.Label(card, text='Antigravity Enterprise App', font=('Inter', 7), fg=PAL['dim'], bg=PAL['panel']).pack()
            btn = tk.Button(card, text='INITIATE', font=('Inter', 7, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=10)
            btn.pack(pady=(10, 0))
            card.bind('<Enter>', lambda e, cd=card: cd.config(highlightbackground=PAL['accent']))
            card.bind('<Leave>', lambda e, cd=card: cd.config(highlightbackground=PAL['border']))