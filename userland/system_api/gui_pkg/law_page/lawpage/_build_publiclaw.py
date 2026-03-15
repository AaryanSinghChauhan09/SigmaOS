# Generated method: LawPage._build_publiclaw
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_publiclaw(self, parent):
        tk.Label(parent, text='Plain Language Law (Nyaaya)', font=FONT_MED, fg=PAL['teal'], bg=PAL['bg']).pack(pady=10)
        tk.Label(parent, text="Explain 'FIR' / 'Bail' / 'Contract' and more.", fg=PAL['dim'], bg=PAL['bg']).pack()