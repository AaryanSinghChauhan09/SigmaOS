# Generated method: PromptOMaticPage.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_LOGO, FONT_MONO

class PromptOMaticPage:
    def __init__(self, parent, gui):
        super().__init__(parent, gui, 'Prompt-o-Matic', 'AI Orchestration Core')
        self._build_content()