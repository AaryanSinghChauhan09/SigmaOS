# Generated method: SigmaQuickSettings.__init__
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def __init__(self, kernel, parent):
        self.kernel = kernel
        self.parent = parent
        self.frame = ttk.Frame(parent, padding=20)
        self._build_ui()