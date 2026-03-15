# Generated method: SovereignLabPage.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MONO

class SovereignLabPage:
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()