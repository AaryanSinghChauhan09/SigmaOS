"""
Auto-split from userland\system_api\gui_pkg\ncert_simulator.py — NcertSimulatorPage.build
"""

import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED



class NcertSimulatorPage:
    def build(self):
        self.controller._build_page_header(self, 'NCERT VIRTUAL LAB (CLASS 1-12)', 'Physics, Chem, Math & Bio Simulation Suite')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        card = self.controller._card(main_panel, 'Simulator Launch Core')
        card.master.pack(pady=50)
        tk.Label(card, text='The NCERT Virtual Lab offers dynamic, browser-based simulations of curriculum experiments for Class 1-12.', font=FONT_MED, bg=PAL['card'], fg=PAL['dim'], wraplength=400).pack(pady=20, padx=20)

        def _launch():
            self._generate_and_launch_html()
            self.controller._notify('NCERT Lab', 'Class 1-12 Virtual Lab launched in browser.', 'OK')
        ttk.Button(card, text='🎓 Launch NCERT Virtual Lab (Class 1-12)', command=_launch, style='Teal.TButton').pack(pady=20)
