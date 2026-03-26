# Generated method: PeriodicTable.__init__
import tkinter as tk
from tkinter import messagebox

class PeriodicTable:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • NCERT Periodic Table Pro')
        self.geometry('1100x700')
        self.configure(bg=PAL['bg'])
        self._build_ui()