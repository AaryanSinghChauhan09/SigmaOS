# Generated method: MissionControlPage.__init__
import tkinter as tk
from tkinter import scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_TITLE

class MissionControlPage:
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()