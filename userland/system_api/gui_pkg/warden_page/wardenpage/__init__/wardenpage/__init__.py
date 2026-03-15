# Generated method: WardenPage.__init__
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class WardenPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Network Warden', 'P2P Mesh & Port Monitor')
        self._build_ui()