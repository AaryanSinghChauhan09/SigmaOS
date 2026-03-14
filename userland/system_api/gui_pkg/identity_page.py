import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class IdentityPage(SigmaPage):
    """🔐 Identity Vault: Decentralized SID Manager"""
    def __init__(self, parent, gui):
        super().__init__(parent, gui, "Identity Vault", "Decentralized SID Manager")
        self._build_ui()

    def _build_ui(self):
        self._card(self, "Sovereign ID: sigma-usr-42-alpha").master.pack(fill="x", pady=20)
        self._card(self, "Biometric Keys: [OK] Fingerprint / Retina / Voice").master.pack(fill="x", pady=5)
