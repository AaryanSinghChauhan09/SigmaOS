# Generated method: CosmosDashPage.__init__
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class CosmosDashPage:
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()