# Generated method: SiloPage.__init__
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class SiloPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Sovereign Silo', 'Hardened Data Vault')
        self._build_ui()