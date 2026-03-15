# Generated method: WardenPage._build_ui
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class WardenPage:
    def _build_ui(self):
        self._console(self, height=25).pack(fill='both', expand=True, pady=10)