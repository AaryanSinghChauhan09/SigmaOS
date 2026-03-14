import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class SiloPage(SigmaPage):
    """🛡️ Sovereign Silo: Hardened Data Vault"""
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Sovereign Silo", "Hardened Data Vault")
        self._build_ui()

    def _build_ui(self):
        self._card(self, "Status: LOCKDOWN ACTIVE").master.pack(fill="x", pady=20)
        self._console(self, height=15).pack(fill="both", expand=True)
