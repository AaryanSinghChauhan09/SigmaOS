# Generated method: LawPage._build_calculators
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_calculators(self, parent):
        tk.Label(parent, text='Statutory Financial Calculators', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(pady=10)
        ttk.Button(parent, text='Calculate FY25 Tax').pack(pady=5)
        ttk.Button(parent, text='Calculate Gratuity').pack(pady=5)