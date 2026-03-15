# Generated method: IntelligenceHubPage._create_tab
import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL

class IntelligenceHubPage:
    def _create_tab(self, notebook, title):
        frame = tk.Frame(notebook, bg=PAL['bg'])
        notebook.add(frame, text=title)
        return frame