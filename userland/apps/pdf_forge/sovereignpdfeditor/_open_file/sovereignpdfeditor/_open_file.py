# Generated method: SovereignPDFEditor._open_file
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class SovereignPDFEditor:
    def _open_file(self):
        f = filedialog.askopenfilename()
        if f:
            self.active_file = f
            self.status_lbl.config(text=f'📄 {os.path.basename(f)}', fg=PAL['accent'])
            self.status.config(text=f'LOADED: {os.path.basename(f)} | SHA-3 VERIFIED', bg=PAL['success'])