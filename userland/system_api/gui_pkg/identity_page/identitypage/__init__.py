# Generated method: IdentityPage.__init__
import tkinter as tk
from .base_page import SigmaPage
from .styles import PAL

class IdentityPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Identity Vault', 'Decentralized SID Manager')
        self._build_ui()