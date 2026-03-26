# Generated method: NumberBaseConverter.__init__
import tkinter as tk
from tkinter import ttk

class NumberBaseConverter:
    def __init__(self, kernel=None):
        super().__init__()
        self.title('SigmaOS Number Base Converter')
        self.geometry('640x560')
        self.configure(bg=PAL['bg'])
        self.resizable(False, False)
        self._build()