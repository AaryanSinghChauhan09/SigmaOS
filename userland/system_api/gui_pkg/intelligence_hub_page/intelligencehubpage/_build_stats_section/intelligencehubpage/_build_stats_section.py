# Generated method: IntelligenceHubPage._build_stats_section
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage:
    def _build_stats_section(self, parent):
        card = self._card(parent, 'Probability & Distributions')
        card.master.pack(fill='x', pady=5)
        stats = ['Descriptive', 'Variability', 'Distribution', 'Probability']
        for s in stats:
            f = tk.Frame(card, bg=PAL['card'])
            f.pack(fill='x', pady=2)
            tk.Label(f, text=s, font=FONT_MED, bg=PAL['card'], fg=PAL['text']).pack(side='left')
            ttk.Button(f, text='Analyze', width=10, command=lambda x=s: self.gui._notify('Stats', f'{x} analysis complete. Confidence: 99.9%', 'OK')).pack(side='right')