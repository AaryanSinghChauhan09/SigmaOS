# Generated method: ExcelValidator._select_file
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class ExcelValidator:
    def _select_file(self):
        f = filedialog.askopenfilename()
        if f:
            self.active_file = f
            self.file_lbl.config(text=f'DATASET: {os.path.basename(f)}', fg=PAL['accent'])
            self.status.config(text=f'ACTIVE: {os.path.basename(f)} | ANALYZING SCHEMA...')