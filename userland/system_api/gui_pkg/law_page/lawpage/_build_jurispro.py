# Generated method: LawPage._build_jurispro
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_jurispro(self, parent):
        tk.Label(parent, text='Jurisprudential Analysis Hub', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(pady=10)
        txt = tk.Text(parent, bg=PAL['bg2'], fg=PAL['gold'], font=FONT_MED, height=15)
        txt.pack(fill='both', expand=True, padx=20, pady=10)
        txt.insert('1.0', 'Select a legal school for analysis...')