import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class WardenPage(SigmaPage):
    """🛰️ Network Warden: P2P Mesh & Port Monitor"""
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Network Warden", "P2P Mesh & Port Monitor")
        self._build_ui()

    def _build_ui(self):
        self._console(self, height=25).pack(fill="both", expand=True, pady=10)
