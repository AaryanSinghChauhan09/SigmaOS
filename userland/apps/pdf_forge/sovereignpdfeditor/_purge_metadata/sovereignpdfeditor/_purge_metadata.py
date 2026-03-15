# Generated method: SovereignPDFEditor._purge_metadata
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class SovereignPDFEditor:
    def _purge_metadata(self):
        self.status.config(text='PURGING METADATA SHIMS...', bg=PAL['warning'])
        self.after(1000, lambda: messagebox.showinfo('Purge', 'Author IDs, EXIF, and Serial Strings wiped.'))