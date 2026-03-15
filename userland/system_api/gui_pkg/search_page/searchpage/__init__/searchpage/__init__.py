# Generated method: SearchPage.__init__
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class SearchPage:
    def __init__(self, parent, gui):
        SigmaPage.__init__(self, parent, gui, 'AERYN SEMANTIC SEARCH', 'Local-First Vector Intelligence Retrieval')
        self.build()