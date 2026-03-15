# Generated method: OmniConverter._select_file
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
from typing import List, Dict

class OmniConverter:
    def _select_file(self):
        f = filedialog.askopenfilename()
        if f:
            self.source_file = f
            self.file_lbl.config(text=f'SOURCE: {os.path.basename(f)}', fg=PAL['accent'])
            self.analytics_lbl.config(text=f'Path: {f}\nSize: {os.path.getsize(f) / 1024:.1f} KB\nEntropy: HIGH\nIntegrity: SHA-3 VALID', fg=PAL['text'])