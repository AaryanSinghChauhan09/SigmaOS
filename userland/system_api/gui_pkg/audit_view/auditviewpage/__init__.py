# Generated method: AuditViewPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class AuditViewPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Titan Parity Audit', 'Verifying SigmaOS Dominance vs Titan OSs')
        self._build_ui()