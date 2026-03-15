# Generated method: IndentFlow.__init__
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random

class IndentFlow:
    def __init__(self):
        super().__init__()
        self.title('Sovereign IndentFlow Apex Pro')
        self.geometry('1150x800')
        self.configure(bg=PAL['bg'])
        self._setup_styles()
        self._build_ui()