# Generated method: SpectralAnalyzer.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random

class SpectralAnalyzer:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Spectral Disk Analyzer')
        self.geometry('1100x750')
        self.configure(bg=PAL['bg'])
        self._setup_styles()
        self._build_ui()