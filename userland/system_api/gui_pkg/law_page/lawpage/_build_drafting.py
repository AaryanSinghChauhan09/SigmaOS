# Generated method: LawPage._build_drafting
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class LawPage:
    def _build_drafting(self, parent):
        tk.Label(parent, text='Legal Drafting Workbench', font=FONT_MED, fg=PAL['gold'], bg=PAL['bg']).pack(pady=10)
        box = tk.Text(parent, font=('Courier New', 10), bg=PAL['bg2'], fg=PAL['text'], height=15)
        box.pack(fill='both', expand=True, padx=20, pady=10)