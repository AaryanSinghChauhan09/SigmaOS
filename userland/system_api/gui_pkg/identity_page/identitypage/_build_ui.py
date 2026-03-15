# Generated method: IdentityPage._build_ui
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class IdentityPage:
    def _build_ui(self):
        self._card(self, 'Sovereign ID: sigma-usr-42-alpha').master.pack(fill='x', pady=20)
        self._card(self, 'Biometric Keys: [OK] Fingerprint / Retina / Voice').master.pack(fill='x', pady=5)