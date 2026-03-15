# Generated method: SiloPage._build_ui
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class SiloPage:
    def _build_ui(self):
        self._card(self, 'Status: LOCKDOWN ACTIVE').master.pack(fill='x', pady=20)
        self._console(self, height=15).pack(fill='both', expand=True)