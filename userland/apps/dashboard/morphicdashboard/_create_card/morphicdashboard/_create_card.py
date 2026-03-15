# Generated method: MorphicDashboard._create_card
import tkinter as tk
from tkinter import ttk, messagebox
import sys, os, time, random
from typing import Dict, Any, List, Optional

class MorphicDashboard:
    def _create_card(self, parent, title, attr, row, col, icon_key='hal'):
        card = tk.Frame(parent, bg=PAL['surface'], padx=30, pady=30, highlightthickness=1, highlightbackground='#2A2D35')
        card.grid(row=row, column=col, sticky='nsew', padx=10, pady=10)
        parent.grid_columnconfigure(col, weight=1)
        parent.grid_rowconfigure(row, weight=1)
        icon_fr = tk.Frame(card, bg=PAL['surface'])
        icon_fr.pack(anchor='w', fill='x')
        icon_lbl = tk.Label(icon_fr, text=ICONS.get(icon_key, '🔹'), font=('Inter Bold', 14), fg=PAL['primary'], bg=PAL['surface'])
        icon_lbl.pack(side='left')
        tk.Label(icon_fr, text=f' {title}', font=('Inter Bold', 10), fg='#8E8E93', bg=PAL['surface']).pack(side='left')
        val = tk.Label(card, text='--', font=('Inter Bold', 36), fg='white', bg=PAL['surface'])
        val.pack(anchor='w', pady=(10, 0))
        setattr(self, attr, val)
        return icon_lbl