"""
Auto-split from userland\system_api\gui_pkg\advanced_calculator.py — AdvancedCalculatorPage.build
"""

import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED



class AdvancedCalculatorPage:
    def build(self):
        self.controller._build_page_header(self, 'ADVANCED WEB CALCULATOR', 'Browser-based IIT JEE & NEET AI Solver')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        card = self.controller._card(main_panel, 'Quantum Launch Core')
        card.master.pack(pady=50)
        tk.Label(card, text='The Advanced Calculator is a high-performance Browser-Based Utility.', font=FONT_MED, bg=PAL['card'], fg=PAL['dim']).pack(pady=20, padx=20)

        def _launch():
            self._generate_and_launch_html()
            self.controller._notify('Calculator', 'Browser-based solver launched.', 'OK')
        ttk.Button(card, text='🚀 Launch IIT JEE & NEET Calculator in Browser', command=_launch, style='Teal.TButton').pack(pady=20)
