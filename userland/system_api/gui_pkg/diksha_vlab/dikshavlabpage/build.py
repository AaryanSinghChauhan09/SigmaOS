"""
Auto-split from userland\system_api\gui_pkg\diksha_vlab.py — DikshaVLabPage.build
"""

import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED



class DikshaVLabPage:
    def build(self):
        self.controller._build_page_header(self, 'NATIONAL VIRTUAL LABS (DIKSHA & OLABS)', 'Swayam/VLab Unified Simulator Core')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        card = self.controller._card(main_panel, 'Online Labs & Learning Engine')
        card.master.pack(pady=50)
        tk.Label(card, text='A comprehensive offline integration of DIKSHA, VLab, and OLabs paradigms. Features Theory, Procedure, Simulator, and Viva Voce for every NCERT curriculum experiment.', font=FONT_MED, bg=PAL['card'], fg=PAL['dim'], wraplength=500, justify='center').pack(pady=20, padx=20)

        def _launch():
            self._generate_and_launch_html()
            self.controller._notify('Virtual Lab', 'Unified National Virtual Labs launched.', 'OK')
        ttk.Button(card, text='🏛️ Launch Integrated Virtual Labs Portal', command=_launch, style='Teal.TButton').pack(pady=20)
