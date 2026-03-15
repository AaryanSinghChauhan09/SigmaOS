# Generated method: BuyhatkePage._build_tracker
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class BuyhatkePage:
    def _build_tracker(self, parent):
        l_fr = tk.Frame(parent, bg=PAL['bg2'], width=300)
        l_fr.pack(side='left', fill='both', padx=5)
        tk.Label(l_fr, text='Product Price Intel', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg2']).pack(pady=10)
        ent = ttk.Entry(l_fr)
        ent.pack(fill='x', padx=10)
        ent.insert(0, 'iPhone 15')
        ttk.Button(l_fr, text='Analyze Trend').pack(pady=10)