"""
Auto-split from userland\system_api\gui_pkg\katbook_reader.py — KatbookReaderPage.build
"""

import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED



class KatbookReaderPage:
    def build(self):
        self.controller._build_page_header(self, 'AI INTERACTIVE NCERT TEXTBOOKS', 'KATBOOK Digital Learning Format')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        card = self.controller._card(main_panel, 'Adaptive Learning Syllabus Reader')
        card.master.pack(pady=50)
        tk.Label(card, text='KATBOOK integrates the NCERT textbook texts directly alongside AI tools like dictionary lookups, multilingual translation, inline videos, and Text-to-Speech audiobooks.', font=FONT_MED, bg=PAL['card'], fg=PAL['dim'], wraplength=500, justify='center').pack(pady=20, padx=20)

        def _launch():
            self._generate_and_launch_html()
            self.controller._notify('KATBOOK', 'AI NCERT Interactive reader launched.', 'OK')
        ttk.Button(card, text='📚 Open KATBOOK Digital Library', command=_launch, style='Teal.TButton').pack(pady=20)
