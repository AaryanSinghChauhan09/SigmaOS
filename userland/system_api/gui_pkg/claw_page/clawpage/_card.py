# Generated method: ClawPage._card
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_MED, FONT_MONO
from sigma_core.ai.sovereign_claw import SovereignClaw

class ClawPage:
    def _card(self, parent, title):
        """Helper to create a styled card."""
        fr = tk.Frame(parent, bg=PAL['card'], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL['border'])
        tk.Label(fr, text=title.upper(), font=FONT_TITLE, fg=PAL['accent'], bg=PAL['card']).pack(anchor='w')
        return fr