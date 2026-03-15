# Generated method: ExcelValidator.__init__
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class ExcelValidator:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Excel Validator')
        self.geometry('1000x750')
        self.configure(bg=PAL['bg'])
        self.active_file = None
        self._build_ui()