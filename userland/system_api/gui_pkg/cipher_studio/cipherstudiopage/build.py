"""
Auto-split from userland\system_api\gui_pkg\cipher_studio.py — CipherStudioPage.build
"""

import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED



class CipherStudioPage:
    def build(self):
        self.controller._build_page_header(self, 'ZERO-TRUST CIPHER STUDIO', 'Browser-based Local Encryption & Hashing')
        main_panel = tk.Frame(self, bg=PAL['bg'])
        main_panel.pack(fill='both', expand=True, padx=20, pady=10)
        card = self.controller._card(main_panel, 'Cryptography Engine Core')
        card.master.pack(pady=50)
        tk.Label(card, text='The Cipher Studio executes all cryptographic operations locally within the browser. No data ever leaves the device.', font=FONT_MED, bg=PAL['card'], fg=PAL['dim'], wraplength=400).pack(pady=20, padx=20)

        def _launch():
            self._generate_and_launch_html()
            self.controller._notify('Cipher Studio', 'Browser encryption suite online.', 'OK')
        ttk.Button(card, text='🔐 Launch Cipher Studio in Browser', command=_launch, style='Teal.TButton').pack(pady=20)
