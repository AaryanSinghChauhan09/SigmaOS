# Generated method: AccessPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL

class AccessPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Omni Access', 'Accessibility & Voice Command')
        self._build_ui()