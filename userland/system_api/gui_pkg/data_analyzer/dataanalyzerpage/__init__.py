# Generated method: DataAnalyzerPage.__init__
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import csv
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class DataAnalyzerPage:
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()