# Generated method: LinuxParityPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Sovereign Linux Bridge', 'Distro Parity Engine')
        self._build_ui()