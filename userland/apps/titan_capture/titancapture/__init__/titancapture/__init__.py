# Generated method: TitanCapture.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class TitanCapture:
    def __init__(self):
        super().__init__()
        self.title('Titan Capture Apex Pro')
        self.geometry('500x450')
        self.configure(bg=PAL['bg'])
        self.attributes('-topmost', True)
        self._recording = False
        self._start_time = 0
        self._setup_styles()
        self._build_ui()