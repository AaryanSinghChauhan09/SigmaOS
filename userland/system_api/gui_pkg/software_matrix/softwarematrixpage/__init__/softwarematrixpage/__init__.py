# Generated method: SoftwareMatrixPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SoftwareMatrixPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Galactic Store', 'Atomic Software Matrix — 0ms Deployments')
        self._build_ui()