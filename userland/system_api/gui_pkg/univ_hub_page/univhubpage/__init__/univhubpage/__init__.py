# Generated method: UnivHubPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO

class UnivHubPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Universal OS Hub', 'Cross-Platform Parity — Absorption of Windows/macOS/Linux')
        self._build_ui()